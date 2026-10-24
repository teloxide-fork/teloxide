use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    pin::pin,
    time::{Duration, Instant},
};

use either::Either;
use futures::{future, FutureExt as _};
use tokio::sync::{mpsc, mpsc::error::TryRecvError, oneshot::Sender};
use vecrem::VecExt;

use crate::{
    adaptors::throttle::{request_lock::RequestLock, ChatIdHash, Limits, Settings},
    errors::AsResponseParameters,
    requests::Requester,
};

const MINUTE: Duration = Duration::from_secs(60);
const SECOND: Duration = Duration::from_secs(1);

// Delay between worker iterations.
//
// For now it's `second/4`, but that number is chosen pretty randomly, we may
// want to change this.
const DELAY: Duration = Duration::from_millis(250);

/// Minimal time between calls to queue_full function
const QUEUE_FULL_DELAY: Duration = Duration::from_secs(4);

#[derive(Debug)]
pub(super) enum InfoMessage {
    GetLimits { response: Sender<Limits> },
    SetLimits { new: Limits, response: Sender<()> },
}

type RequestsSent = u32;

// I wish there was special data structure for history which removed the
// need in 2 hashmaps
// (waffle)
#[derive(Default)]
struct RequestsSentToChats {
    per_min: HashMap<ChatIdHash, RequestsSent>,
    per_sec: HashMap<ChatIdHash, RequestsSent>,
}

pub(super) struct FreezeUntil {
    pub(super) until: Instant,
    pub(super) after: Duration,
    pub(super) chat: ChatIdHash,
}

// Throttling is quite complicated. This comment describes the algorithm of the
// current implementation.
//
// ### Request
//
// When a throttling request is sent, it sends a tuple of `ChatId` and
// `Sender<()>` to the worker. Then the request waits for a notification from
// the worker. When notification is received, it sends the underlying request.
//
// ### Worker
//
// The worker does the most important job -- it ensures that the limits are
// never exceeded.
//
// The worker stores a history of requests sent in the last minute (and to which
// chats they were sent) and a queue of pending updates.
//
// The worker does the following algorithm loop:
//
// 1. If the queue is empty, wait for the first message in incoming channel (and
// add it to the queue).
//
// 2. Read all present messages from an incoming channel and transfer them to
// the queue.
//
// 3. Record the current time.
//
// 4. Clear the history from records whose time < (current time - minute).
//
// 5. Count all requests which were sent last second, `allowed =
// limit.messages_per_sec_overall - count`.
//
// 6. If `allowed == 0` wait a bit and `continue` to the next iteration.
//
// 7. Count how many requests were sent to which chats (i.e.: create
// `Map<ChatId, Count>`). (Note: the same map, but for last minute also exists,
// but it's updated, instead of recreation.)
//
// 8. While `allowed >= 0` search for requests which chat haven't exceed the
// limits (i.e.: map[chat] < limit), if one is found, decrease `allowed`, notify
// the request that it can be now executed, increase counts, add record to the
// history.
pub(super) async fn worker<B>(
    Settings { mut limits, mut on_queue_full, retry, check_slow_mode }: Settings,
    mut rx: mpsc::Receiver<(ChatIdHash, RequestLock)>,
    mut info_rx: mpsc::Receiver<InfoMessage>,
    bot: B,
) where
    B: Requester,
    B::Err: AsResponseParameters,
{
    // FIXME(waffle): Make an research about data structures for this queue.
    //                Currently this is O(n) removing (n = number of elements
    //                stayed), amortized O(1) push (vec+vecrem).
    let mut queue: Vec<(ChatIdHash, RequestLock)> =
        Vec::with_capacity(limits.messages_per_sec_overall as usize);

    let mut history: VecDeque<(ChatIdHash, Instant)> = VecDeque::new();
    let mut requests_sent = RequestsSentToChats::default();

    let mut slow_mode: Option<HashMap<ChatIdHash, (Duration, Instant)>> =
        check_slow_mode.then(HashMap::new);

    let mut rx_is_closed = false;

    let mut last_queue_full =
        Instant::now().checked_sub(QUEUE_FULL_DELAY).unwrap_or_else(Instant::now);

    let (freeze_tx, mut freeze_rx) = mpsc::channel::<FreezeUntil>(1);

    while !rx_is_closed || !queue.is_empty() {
        // FIXME(waffle):
        // 1. If the `queue` is empty, `read_from_rx` call down below will 'block'
        //    execution until a request is sent. While the execution is 'blocked' no
        //    `InfoMessage`s could be answered.
        //
        // 2. If limits are decreased, ideally we want to shrink queue.
        //
        // *blocked in asynchronous way
        answer_info(&mut info_rx, &mut limits);

        loop {
            let res = future::select(
                pin!(freeze_rx.recv()),
                pin!(read_from_rx(&mut rx, &mut queue, &mut rx_is_closed)),
            )
            .map(either)
            .await
            .map_either(|l| l.0, |r| r.0);

            match res {
                Either::Left(freeze_until) => {
                    freeze(&mut freeze_rx, slow_mode.as_mut(), &bot, freeze_until).await;
                }
                Either::Right(()) => break,
            }
        }
        //debug_assert_eq!(queue.capacity(), limits.messages_per_sec_overall as usize);

        if queue.len() == queue.capacity() && last_queue_full.elapsed() > QUEUE_FULL_DELAY {
            last_queue_full = Instant::now();
            tokio::spawn(on_queue_full(queue.len()));
        }

        // _Maybe_ we need to use `spawn_blocking` here, because there is
        // decent amount of blocking work. However _for now_ I've decided not
        // to use it here.
        //
        // Reasons (not to use `spawn_blocking`):
        //
        // 1. The work seems not very CPU-bound, it's not heavy computations, it's more
        //    like light computations.
        //
        // 2. `spawn_blocking` is not zero-cost — it spawns a new system thread
        //    + do so other work. This may actually be *worse* then current
        //    "just do everything in this async fn" approach.
        //
        // 3. With `rt-threaded` feature, tokio uses [`num_cpus()`] threads which should
        //    be enough to work fine with one a-bit-blocking task. Crucially current
        //    behaviour will be problem mostly with single-threaded runtimes (and in
        //    case you're using one, you probably don't want to spawn unnecessary
        //    threads anyway).
        //
        // I think if we'll ever change this behaviour, we need to make it
        // _configurable_.
        //
        // See also [discussion (ru)].
        //
        // NOTE: If you are reading this because you have any problems because
        // of this worker, open an [issue on github]
        //
        // [`num_cpus()`]: https://vee.gg/JGwq2
        // [discussion (ru)]: https://t.me/rust_async/27891
        // [issue on github]: https://github.com/teloxide/teloxide/issues/new
        //
        // (waffle)

        let now = Instant::now();
        let min_back = now.checked_sub(MINUTE).unwrap_or(now);
        let sec_back = now.checked_sub(SECOND).unwrap_or(now);

        // make history and requests_sent up-to-date
        while let Some((_, time)) = history.front() {
            // history is sorted, we found first up-to-date thing
            if time >= &min_back {
                break;
            }

            if let Some((chat, _)) = history.pop_front() {
                let entry = requests_sent.per_min.entry(chat).and_modify(|count| {
                    *count -= 1;
                });

                if let Entry::Occupied(entry) = entry {
                    if *entry.get() == 0 {
                        entry.remove_entry();
                    }
                }
            }
        }

        // as truncates which is ok since in case of truncation it would always be >=
        // limits.overall_s
        let used = history.iter().rev().take_while(|(_, time)| time > &sec_back).count() as u32;
        let mut allowed = limits.messages_per_sec_overall.saturating_sub(used);

        if allowed == 0 {
            requests_sent.per_sec.clear();
            tokio::time::sleep(DELAY).await;
            continue;
        }

        for (chat, _) in history.iter().rev().take_while(|(_, time)| time > &sec_back) {
            *requests_sent.per_sec.entry(*chat).or_insert(0) += 1;
        }

        let mut queue_removing = queue.removing();

        while let Some(entry) = queue_removing.next() {
            let chat = &entry.value().0;

            let slow_mode = slow_mode.as_mut().and_then(|sm| sm.get_mut(chat));

            if let Some(&mut (delay, last)) = slow_mode {
                if last + delay > Instant::now() {
                    continue;
                }
            }

            let requests_sent_per_sec_count = requests_sent.per_sec.get(chat).copied().unwrap_or(0);
            let requests_sent_per_min_count = requests_sent.per_min.get(chat).copied().unwrap_or(0);

            let messages_per_min_limit = if chat.is_channel_or_supergroup() {
                limits.messages_per_min_channel_or_supergroup
            } else {
                limits.messages_per_min_chat
            };

            let limits_not_exceeded = requests_sent_per_sec_count < limits.messages_per_sec_chat
                && requests_sent_per_min_count < messages_per_min_limit;

            if limits_not_exceeded {
                // Unlock the associated request.

                let chat = *chat;
                let (_, lock) = entry.remove();

                // Only count request as sent if the request wasn't dropped before unlocked
                if lock.unlock(retry, freeze_tx.clone()).is_ok() {
                    *requests_sent.per_sec.entry(chat).or_insert(0) += 1;
                    *requests_sent.per_min.entry(chat).or_insert(0) += 1;
                    history.push_back((chat, Instant::now()));

                    if let Some((_, last)) = slow_mode {
                        *last = Instant::now();
                    }

                    // We have "sent" one request, so now we can send one less.
                    allowed -= 1;
                    if allowed == 0 {
                        break;
                    }
                }
            }
        }

        // It's easier to just recompute last second stats, instead of keeping
        // track of it alongside with minute stats, so we just throw this away.
        requests_sent.per_sec.clear();
        tokio::time::sleep(DELAY).await;
    }
}

fn answer_info(rx: &mut mpsc::Receiver<InfoMessage>, limits: &mut Limits) {
    while let Ok(req) = rx.try_recv() {
        // Errors are ignored with .ok(). Error means that the response channel
        // is closed and the response isn't needed.
        match req {
            InfoMessage::GetLimits { response } => response.send(*limits).ok(),
            InfoMessage::SetLimits { new, response } => {
                *limits = new;
                response.send(()).ok()
            }
        };
    }
}

// FIXME: https://github.com/rust-lang/rust-clippy/issues/11610
#[allow(clippy::needless_pass_by_ref_mut)]
async fn freeze(
    rx: &mut mpsc::Receiver<FreezeUntil>,
    mut slow_mode: Option<&mut HashMap<ChatIdHash, (Duration, Instant)>>,
    bot: &impl Requester,
    mut imm: Option<FreezeUntil>,
) {
    while let Some(freeze_until) = imm.take().or_else(|| rx.try_recv().ok()) {
        let FreezeUntil { until, after, chat } = freeze_until;
        let mut slow_mode_enabled_and_likely_the_cause = false;

        // Clippy thinks that this `.as_deref_mut()` doesn't change the type (&mut
        // HashMap -> &mut HashMap), but it's actually a reborrow (the lifetimes
        // differ), since we are in a loop, simply using `slow_mode` would produce a
        // moved-out error.
        #[allow(clippy::needless_option_as_deref)]
        if let Some(slow_mode) = slow_mode.as_deref_mut() {
            // TODO: do something with channels?...
            if let hash @ ChatIdHash::Id(id) = chat {
                // TODO: maybe not call `get_chat` every time?

                // At this point there isn't much we can do with the error besides ignoring
                if let Ok(chat) = bot.get_chat(id).await {
                    match chat.slow_mode_delay() {
                        Some(delay) => {
                            let now = Instant::now();
                            let new_delay = delay.duration();
                            slow_mode.insert(hash, (new_delay, now));
                            slow_mode_enabled_and_likely_the_cause =
                                !new_delay.is_zero() && after <= new_delay;
                        }
                        None => {
                            slow_mode.remove(&hash);
                        }
                    };
                }
            }
        }

        // Only a successful current lookup with a positive slow mode interval can
        // explain a remaining wait no longer than that interval. Never use stale
        // cache entries to skip the global wait after a failed lookup.
        // This is a heuristic: a short global RetryAfter can also fit the interval.
        if !slow_mode_enabled_and_likely_the_cause {
            log::warn!(
                "freezing the bot for approximately {after:?} due to `RetryAfter` error from \
                 telegram"
            );

            tokio::time::sleep_until(until.into()).await;

            log::warn!("unfreezing the bot");
        }
    }
}

async fn read_from_rx<T>(rx: &mut mpsc::Receiver<T>, queue: &mut Vec<T>, rx_is_closed: &mut bool) {
    if queue.is_empty() {
        log::debug!("blocking on queue");

        match rx.recv().await {
            Some(req) => queue.push(req),
            None => *rx_is_closed = true,
        }
    }

    // Don't grow queue bigger than the capacity to limit DOS possibility
    while queue.len() < queue.capacity() {
        match rx.try_recv() {
            Ok(req) => queue.push(req),
            Err(TryRecvError::Disconnected) => {
                *rx_is_closed = true;
                break;
            }
            // There are no items in queue.
            Err(TryRecvError::Empty) => break,
        }
    }
}

fn either<L, R>(x: future::Either<L, R>) -> Either<L, R> {
    match x {
        future::Either::Left(l) => Either::Left(l),
        future::Either::Right(r) => Either::Right(r),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        sync::{
            atomic::{AtomicBool, AtomicUsize, Ordering},
            Arc,
        },
        thread,
    };

    use serde_json::{json, Value};

    use crate::{
        adaptors::throttle::{channel, worker::*, Throttle, ThrottlingRequest},
        requests::{Request, Requester},
        types::ChatId,
        Bot,
    };

    // Exercise the real Bot API serialization/deserialization without another mock
    // dependency. Every connection receives the configured getChat response.
    struct MockApi {
        bot: Bot,
        calls: Arc<AtomicUsize>,
        stop: Arc<AtomicBool>,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl MockApi {
        fn new(response: Value) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            listener.set_nonblocking(true).unwrap();
            let calls = Arc::new(AtomicUsize::new(0));
            let stop = Arc::new(AtomicBool::new(false));
            let thread = {
                let calls = Arc::clone(&calls);
                let stop = Arc::clone(&stop);
                thread::spawn(move || {
                    while !stop.load(Ordering::SeqCst) {
                        let (mut stream, _) = match listener.accept() {
                            Ok(connection) => connection,
                            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                                thread::sleep(Duration::from_millis(1));
                                continue;
                            }
                            Err(error) => panic!("mock accept: {error}"),
                        };
                        stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
                        let mut request = Vec::new();
                        let mut buffer = [0; 4096];
                        loop {
                            let n = stream.read(&mut buffer).unwrap();
                            assert_ne!(n, 0);
                            request.extend_from_slice(&buffer[..n]);
                            if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                                let headers = String::from_utf8_lossy(&request[..end]);
                                let length: usize = headers
                                    .lines()
                                    .find_map(|line| {
                                        line.to_ascii_lowercase()
                                            .strip_prefix("content-length:")
                                            .map(|n| n.trim().parse().unwrap())
                                    })
                                    .unwrap_or(0);
                                if request.len() >= end + 4 + length {
                                    let path = headers.lines().next().unwrap();
                                    assert!(
                                        path.contains("/GetChat") || path.contains("/SendMessage")
                                    );
                                    break;
                                }
                            }
                        }
                        calls.fetch_add(1, Ordering::SeqCst);
                        let body = response.to_string();
                        write!(
                            stream,
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: \
                             {}\r\nConnection: close\r\n\r\n{body}",
                            body.len()
                        )
                        .unwrap();
                    }
                })
            };
            Self {
                bot: Bot::new("123:test")
                    .set_api_url(format!("http://{address}/").parse().unwrap()),
                calls,
                stop,
                thread: Some(thread),
            }
        }
    }

    impl Drop for MockApi {
        fn drop(&mut self) {
            self.stop.store(true, Ordering::SeqCst);
            self.thread.take().unwrap().join().unwrap();
        }
    }

    fn chat_response(delay: Option<u32>) -> Value {
        let mut chat = json!({
            "id": -1000000000001_i64, "type": "supergroup", "title": "slow chat",
            "max_reaction_count": 1,
            "accepted_gift_types": {
                "unlimited_gifts": false, "limited_gifts": false,
                "unique_gifts": false, "premium_subscription": false
            }
        });
        if let Some(delay) = delay {
            chat["slow_mode_delay"] = json!(delay);
        }
        // Fail clearly if a fixture no longer represents a valid API response.
        serde_json::from_value::<crate::types::ChatFullInfo>(chat.clone()).unwrap();
        json!({"ok": true, "result": chat})
    }

    // Keep each case's inputs and expected observable effects together.
    #[allow(clippy::too_many_arguments)]
    async fn check_freeze(
        response: Value,
        after: u64,
        enabled: bool,
        stale: bool,
        chat: ChatIdHash,
        skips_global_wait: bool,
        expected_delay: Option<u64>,
        expected_calls: usize,
    ) {
        let api = MockApi::new(response);
        let mut cache = HashMap::new();
        if stale {
            cache.insert(chat, (Duration::from_secs(900), Instant::now()));
        }
        let (_tx, mut rx) = mpsc::channel(1);
        // Classification uses the real retry duration; the synthetic deadline
        // lets us observe global sleeping without waiting fifteen minutes.
        let event = FreezeUntil {
            until: Instant::now() + Duration::from_secs(60),
            after: Duration::from_secs(after),
            chat,
        };
        let result = tokio::time::timeout(
            Duration::from_millis(300),
            freeze(&mut rx, enabled.then_some(&mut cache), &api.bot, Some(event)),
        )
        .await;
        assert_eq!(result.is_ok(), skips_global_wait, "retry_after={after}");
        assert_eq!(api.calls.load(Ordering::SeqCst), expected_calls);
        assert_eq!(cache.get(&chat).map(|(delay, _)| delay.as_secs()), expected_delay);
    }

    const CHAT: ChatIdHash = ChatIdHash::Id(ChatId(-1000000000001));

    #[tokio::test]
    async fn slow_mode_remaining_wait_is_chat_local() {
        for after in [899, 900, 30] {
            check_freeze(chat_response(Some(900)), after, true, false, CHAT, true, Some(900), 1)
                .await;
        }
    }

    #[tokio::test]
    async fn slow_mode_longer_wait_remains_global() {
        for after in [901, 1800] {
            check_freeze(chat_response(Some(900)), after, true, false, CHAT, false, Some(900), 1)
                .await;
        }
    }

    #[tokio::test]
    async fn slow_mode_disabled_does_not_lookup() {
        check_freeze(chat_response(Some(900)), 899, false, false, CHAT, false, None, 0).await;
    }

    #[tokio::test]
    async fn slow_mode_missing_delay_removes_cache() {
        check_freeze(chat_response(None), 899, true, true, CHAT, false, None, 1).await;
    }

    #[tokio::test]
    async fn slow_mode_failed_lookup_cannot_use_stale_cache() {
        for stale in [false, true] {
            check_freeze(
                json!({"ok": false, "error_code": 400, "description": "Bad Request: chat not found"}),
                900, true, stale, CHAT, false, stale.then_some(900), 1,
            ).await;
        }
    }

    #[tokio::test]
    async fn slow_mode_zero_delay_remains_global() {
        check_freeze(chat_response(Some(0)), 1, true, false, CHAT, false, Some(0), 1).await;
    }

    #[tokio::test]
    async fn slow_mode_username_does_not_lookup_or_use_cache() {
        check_freeze(
            chat_response(Some(900)),
            900,
            true,
            true,
            ChatIdHash::ChannelUsernameHash(42),
            false,
            Some(900),
            0,
        )
        .await;
    }

    #[tokio::test]
    async fn slow_mode_worker_allows_other_chat_while_affected_chat_stays_paced() {
        let api = MockApi::new(chat_response(Some(900)));
        let (throttle, worker) = Throttle::with_settings(
            api.bot.clone(),
            Settings { check_slow_mode: true, ..Settings::default() },
        );
        let worker = tokio::spawn(worker);
        let (first, wait) = channel();
        throttle.queue.send((CHAT, first)).await.unwrap();
        let (_, freeze) = wait.await;
        freeze
            .send(FreezeUntil {
                until: Instant::now() + Duration::from_secs(899),
                after: Duration::from_secs(899),
                chat: CHAT,
            })
            .await
            .unwrap();

        let (affected, mut affected_wait) = channel();
        let (other, other_wait) = channel();
        throttle.queue.send((CHAT, affected)).await.unwrap();
        throttle.queue.send((ChatIdHash::Id(ChatId(2)), other)).await.unwrap();
        tokio::time::timeout(Duration::from_secs(2), other_wait)
            .await
            .expect("another chat must not be globally frozen");
        assert_eq!(api.calls.load(Ordering::SeqCst), 1);
        assert!(tokio::time::timeout(Duration::from_millis(300), &mut affected_wait)
            .await
            .is_err());
        worker.abort();
        assert!(worker.await.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    async fn slow_mode_request_keeps_its_own_retry_deadline() {
        let api = MockApi::new(json!({
            "ok": false, "error_code": 429, "description": "Too Many Requests: retry after 1",
            "parameters": {"retry_after": 1}
        }));
        let (tx, mut rx) = mpsc::channel(1);
        let request = ThrottlingRequest {
            request: Arc::new(api.bot.send_message(ChatId(1), "test")),
            chat_id: |_| CHAT,
            worker: tx,
        };
        let task = tokio::spawn(request.send());
        let (_, lock) = rx.recv().await.unwrap();
        let (freeze_tx, mut freeze_rx) = mpsc::channel(1);
        lock.unlock(true, freeze_tx).unwrap();
        let event =
            tokio::time::timeout(Duration::from_secs(2), freeze_rx.recv()).await.unwrap().unwrap();
        assert_eq!(event.after, Duration::from_secs(1));
        // Even if the worker immediately handles the freeze as chat-local, the
        // request must not requeue before its own retry deadline.
        assert!(tokio::time::timeout(Duration::from_millis(150), rx.recv()).await.is_err());
        let (_, lock) =
            tokio::time::timeout(Duration::from_secs(2), rx.recv()).await.unwrap().unwrap();
        assert!(Instant::now() >= event.until);
        let (freeze_tx, mut freeze_rx) = mpsc::channel(1);
        lock.unlock(false, freeze_tx).unwrap();
        freeze_rx.recv().await.unwrap();
        assert!(task.await.unwrap().is_err());
        assert_eq!(api.calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn issue_535() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        // Close channel
        drop(tx);

        // Previously this caused an infinite loop
        super::read_from_rx::<()>(&mut rx, &mut Vec::new(), &mut false).await;
    }
}
