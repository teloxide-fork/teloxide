use std::future::IntoFuture;

use url::Url;

use crate::{
    payloads::{
        AnswerInlineQuery, AnswerWebAppQuery, CopyMessage, EditMessageCaption,
        EditMessageCaptionInline, EditMessageChecklist, EditMessageMedia, EditMessageMediaInline,
        EditMessageText, EditMessageTextInline, EditStory, GiftPremiumSubscription, PostStory,
        SavePreparedInlineMessage, SendAnimation, SendAudio, SendChecklist, SendDocument, SendGift,
        SendGiftChat, SendLivePhoto, SendMediaGroup, SendMessage, SendMessageDraft, SendPaidMedia,
        SendPhoto, SendPoll, SendVideo, SendVoice,
    },
    prelude::Requester,
    requests::{HasPayload, Output, Request},
    types::*,
};

/// Default parse mode adaptor, see
/// [`RequesterExt::parse_mode`](crate::requests::RequesterExt::parse_mode).
#[derive(Clone, Debug)]
pub struct DefaultParseMode<B> {
    bot: B,
    mode: ParseMode,
}

/// Request returned by [`DefaultParseMode`] methods.
#[derive(Clone)]
pub struct DefaultParseModeRequest<R> {
    req: R,
    mode: ParseMode,
}

impl<B> DefaultParseMode<B> {
    /// Creates new [`DefaultParseMode`].
    ///
    /// Note: it's recommended to use [`RequesterExt::parse_mode`] instead.
    ///
    /// [`RequesterExt::parse_mode`]: crate::requests::RequesterExt::parse_mode
    pub fn new(bot: B, parse_mode: ParseMode) -> Self {
        Self { bot, mode: parse_mode }
    }

    /// Allows to access the inner bot.
    pub fn inner(&self) -> &B {
        &self.bot
    }

    /// Unwraps the inner bot.
    pub fn into_inner(self) -> B {
        self.bot
    }

    /// Returns currently used [`ParseMode`].
    pub fn parse_mode(&self) -> ParseMode {
        self.mode
    }
}

impl<R> Request for DefaultParseModeRequest<R>
where
    R: Request + Clone,
    R::Payload: VisitParseModes,
{
    type Err = R::Err;
    type Send = R::Send;
    type SendRef = R::Send;

    // Required methods
    fn send(mut self) -> Self::Send {
        self.req.payload_mut().visit_parse_modes(|mode| _ = mode.get_or_insert(self.mode));
        self.req.send()
    }

    fn send_ref(&self) -> Self::SendRef {
        // There is no other way to change the payload, given a `&self` :(
        self.clone().send()
    }
}

impl<R> IntoFuture for DefaultParseModeRequest<R>
where
    Self: Request,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

impl<R> HasPayload for DefaultParseModeRequest<R>
where
    R: Request,
{
    type Payload = R::Payload;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.req.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.req.payload_ref()
    }
}

macro_rules! f {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        {
            let req = $this.inner().$m($($arg),*);
            DefaultParseModeRequest { req, mode: $this.mode }
        }
    };
}

macro_rules! fty {
    ($T:ident) => {
        DefaultParseModeRequest<B::$T>
    };
}

macro_rules! ftyid {
    ($T:ident) => {
        B::$T
    };
}

macro_rules! fid {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner().$m($($arg),*)
    };
}

impl<B> Requester for DefaultParseMode<B>
where
    B: Requester,
    B::SendMessage: Clone,
    B::SendMessageDraft: Clone,
    B::SendPhoto: Clone,
    B::SendLivePhoto: Clone,
    B::SendVideo: Clone,
    B::SendAudio: Clone,
    B::SendDocument: Clone,
    B::SendAnimation: Clone,
    B::SendVoice: Clone,
    B::EditMessageText: Clone,
    B::EditMessageTextInline: Clone,
    B::EditMessageCaption: Clone,
    B::EditMessageCaptionInline: Clone,
    B::SendPoll: Clone,
    B::SendChecklist: Clone,
    B::CopyMessage: Clone,
    B::PostStory: Clone,
    B::EditStory: Clone,
    B::AnswerInlineQuery: Clone,
    B::AnswerWebAppQuery: Clone,
    B::SavePreparedInlineMessage: Clone,
    B::EditMessageMedia: Clone,
    B::EditMessageMediaInline: Clone,
    B::EditMessageChecklist: Clone,
    B::SendPaidMedia: Clone,
    B::SendMediaGroup: Clone,
    B::GiftPremiumSubscription: Clone,
    B::SendGift: Clone,
    B::SendGiftChat: Clone,
{
    type Err = B::Err;

    requester_forward! {
        send_message,
        send_message_draft,
        send_photo,
        send_live_photo,
        send_video,
        send_audio,
        send_document,
        send_animation,
        send_voice,
        send_poll,
        send_checklist,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_checklist,
        copy_message,
        post_story,
        edit_story,
        answer_inline_query,
        answer_web_app_query,
        save_prepared_inline_message,
        send_paid_media,
        send_media_group,
        edit_message_media,
        edit_message_media_inline,
        gift_premium_subscription,
        send_gift,
        send_gift_chat,
        => f, fty
    }

    requester_forward! {
        get_managed_bot_token,
        get_managed_bot_access_settings,
        set_managed_bot_access_settings,
        replace_managed_bot_token,
        save_prepared_keyboard_button,
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_messages,
        send_video_note,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_dice,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        set_user_emoji_status,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        set_chat_member_tag,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        create_chat_subscription_invite_link,
        edit_chat_subscription_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        get_user_personal_chat_messages,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        answer_guest_query,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        approve_suggested_post,
        decline_suggested_post,
        delete_message,
        delete_messages,
        delete_message_reaction,
        delete_all_message_reactions,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        get_available_gifts,
        verify_user,
        verify_chat,
        remove_user_verification,
        remove_chat_verification,
        read_business_message,
        delete_business_messages,
        set_business_account_name,
        set_business_account_username,
        set_business_account_bio,
        set_business_account_profile_photo,
        remove_business_account_profile_photo,
        set_business_account_gift_settings,
        get_business_account_star_balance,
        transfer_business_account_stars,
        get_business_account_gifts,
        get_user_gifts,
        get_user_profile_audios,
        get_chat_gifts,
        set_my_profile_photo,
        remove_my_profile_photo,
        convert_gift_to_stars,
        upgrade_gift,
        transfer_gift,
        repost_story,
        delete_story,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        get_my_star_balance,
        get_star_transactions,
        refund_star_payment,
        edit_user_star_subscription,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fid, ftyid
    }
}

download_forward! {
    B
    DefaultParseMode<B>
    { this => this.inner() }
}

trait VisitParseModes {
    fn visit_parse_modes(&mut self, visitor: impl FnMut(&mut Option<ParseMode>));
}

macro_rules! impl_visit_parse_modes {
    (
        $(
            $T:ty => [
                $(
                    $field:ident
                ),*
            ]
            ,
        )*
    ) => {
        $(
            impl VisitParseModes for $T {
                fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
                    $(
                        visitor(&mut self.$field);
                    )*
                }
            }
        )*
    }
}

impl_visit_parse_modes! {
    SendMessage => [parse_mode],
    SendMessageDraft => [parse_mode],
    SendPhoto => [parse_mode],
    SendLivePhoto => [parse_mode],
    SendVideo => [parse_mode],
    SendAudio => [parse_mode],
    SendDocument => [parse_mode],
    SendAnimation => [parse_mode],
    SendVoice => [parse_mode],
    EditMessageText => [parse_mode],
    EditMessageTextInline => [parse_mode],
    EditMessageCaption => [parse_mode],
    EditMessageCaptionInline => [parse_mode],
    SendPaidMedia => [parse_mode],
    GiftPremiumSubscription => [text_parse_mode],
    SendGift => [text_parse_mode],
    SendGiftChat => [text_parse_mode],
    // FIXME: check if `parse_mode` changes anything if `.caption` is not set
    //        (and if it does, maybe not call visitor if `self.caption.is_none()`)
    CopyMessage => [parse_mode],
    PostStory => [parse_mode],
    EditStory => [parse_mode],
}

impl VisitParseModes for SendPoll {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        if self.question_entities.is_none() {
            visitor(&mut self.question_parse_mode);
        }

        self.options
            .iter_mut()
            .for_each(|option| visit_parse_modes_in_input_poll_option(option, &mut visitor));

        if self.explanation_entities.is_none() {
            visitor(&mut self.explanation_parse_mode);
        }
        if let Some(media) = &mut self.explanation_media {
            visit_parse_modes_in_input_poll_media(media, &mut visitor);
        }

        if self.description_entities.is_none() {
            visitor(&mut self.description_parse_mode);
        }
        if let Some(media) = &mut self.media {
            visit_parse_modes_in_input_poll_media(media, &mut visitor);
        }
    }
}

impl VisitParseModes for AnswerInlineQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.results
            .iter_mut()
            .for_each(|result| visit_parse_modes_in_inline_query_result(result, &mut visitor))
    }
}

impl VisitParseModes for AnswerWebAppQuery {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SavePreparedInlineMessage {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_inline_query_result(&mut self.result, &mut visitor);
    }
}

impl VisitParseModes for SendMediaGroup {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        self.media
            .iter_mut()
            .for_each(|media| visit_parse_modes_in_input_media(media, &mut visitor))
    }
}

impl VisitParseModes for EditMessageMedia {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageMediaInline {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visit_parse_modes_in_input_media(&mut self.media, &mut visitor);
    }
}

impl VisitParseModes for EditMessageChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

impl VisitParseModes for SendChecklist {
    fn visit_parse_modes(&mut self, mut visitor: impl FnMut(&mut Option<ParseMode>)) {
        visitor(&mut self.checklist.parse_mode);
    }
}

fn visit_parse_modes_in_inline_query_result(
    result: &mut InlineQueryResult,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InlineQueryResult::*;

    let parse_mode = match result {
        // Simply contain `parse_mode`
        CachedAudio(r) => &mut r.parse_mode,
        CachedDocument(r) => &mut r.parse_mode,
        CachedGif(r) => &mut r.parse_mode,
        CachedMpeg4Gif(r) => &mut r.parse_mode,
        CachedPhoto(r) => &mut r.parse_mode,
        CachedVideo(r) => &mut r.parse_mode,
        CachedVoice(r) => &mut r.parse_mode,
        Audio(r) => &mut r.parse_mode,
        Document(r) => &mut r.parse_mode,
        Gif(r) => &mut r.parse_mode,
        Mpeg4Gif(r) => &mut r.parse_mode,
        Photo(r) => &mut r.parse_mode,
        Video(r) => &mut r.parse_mode,
        Voice(r) => &mut r.parse_mode,

        // Can contain parse mode if `InputMessageContent::Text`
        CachedSticker(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Article(r) => match &mut r.input_message_content {
            InputMessageContent::Text(t) => &mut t.parse_mode,
            _ => return,
        },
        Contact(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Location(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },
        Venue(r) => match &mut r.input_message_content {
            Some(InputMessageContent::Text(t)) => &mut t.parse_mode,
            _ => return,
        },

        // Can't contain `parse_mode` at all
        Game(_r) => return,
    };

    visitor(parse_mode);
}

fn visit_parse_modes_in_input_media(
    media: &mut InputMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputMedia::*;

    let parse_mode = match media {
        Photo(m) => &mut m.parse_mode,
        Video(m) => &mut m.parse_mode,
        Animation(m) => &mut m.parse_mode,
        Audio(m) => &mut m.parse_mode,
        Document(m) => &mut m.parse_mode,
        LivePhoto(m) => &mut m.parse_mode,
    };

    visitor(parse_mode);
}

fn visit_parse_modes_in_input_poll_option(
    option: &mut InputPollOption,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputPollOptionFormatting::*;

    match &mut option.formatting {
        Some(TextParseMode(_)) | Some(TextEntities(_)) => {}
        None => {
            let mut parse_mode = None;
            visitor(&mut parse_mode);
            option.formatting = parse_mode.map(TextParseMode);
        }
    }

    if let Some(media) = &mut option.media {
        visit_parse_modes_in_input_poll_option_media(media, visitor);
    }
}

fn visit_parse_modes_in_input_poll_media(
    media: &mut InputPollMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputPollMedia::*;

    match media {
        Animation(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Audio(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Document(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        LivePhoto(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Photo(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Video(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Location(_) | Venue(_) => {}
    }
}

fn visit_parse_modes_in_input_poll_option_media(
    media: &mut InputPollOptionMedia,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    use InputPollOptionMedia::*;

    match media {
        Animation(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        LivePhoto(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Photo(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Video(media) => visit_parse_mode_in_input_media_caption(
            &mut media.parse_mode,
            &media.caption_entities,
            visitor,
        ),
        Location(_) | Sticker(_) | Venue(_) => {}
    }
}

fn visit_parse_mode_in_input_media_caption(
    parse_mode: &mut Option<ParseMode>,
    caption_entities: &Option<Vec<MessageEntity>>,
    visitor: &mut impl FnMut(&mut Option<ParseMode>),
) {
    if caption_entities.is_none() {
        visitor(parse_mode);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_default_parse_mode(parse_mode: &mut Option<ParseMode>) {
        _ = parse_mode.get_or_insert(ParseMode::Html);
    }

    fn media() -> InputFile {
        InputFile::file_id("media".into())
    }

    #[test]
    fn send_poll_visits_new_text_parse_modes() {
        let mut poll = SendPoll::new(
            ChatId(1),
            "question",
            [InputPollOption::new("option"), InputPollOption::new("second")],
        );

        poll.visit_parse_modes(set_default_parse_mode);

        assert_eq!(poll.question_parse_mode, Some(ParseMode::Html));
        assert_eq!(poll.description_parse_mode, Some(ParseMode::Html));
        assert!(matches!(
            poll.options[0].formatting,
            Some(InputPollOptionFormatting::TextParseMode(ParseMode::Html))
        ));
    }

    #[test]
    fn send_poll_visits_new_media_caption_parse_modes() {
        let mut poll = SendPoll::new(
            ChatId(1),
            "question",
            [InputPollOption::new("option")
                .media(InputPollOptionMedia::Photo(InputMediaPhoto::new(media())))],
        );
        poll.explanation_media = Some(InputPollMedia::Audio(InputMediaAudio::new(media())));
        poll.media = Some(InputPollMedia::Video(InputMediaVideo::new(media())));

        poll.visit_parse_modes(set_default_parse_mode);

        let Some(InputPollOptionMedia::Photo(option_media)) = &poll.options[0].media else {
            panic!("expected photo option media");
        };
        assert_eq!(option_media.parse_mode, Some(ParseMode::Html));
        let Some(InputPollMedia::Audio(explanation_media)) = &poll.explanation_media else {
            panic!("expected audio explanation media");
        };
        assert_eq!(explanation_media.parse_mode, Some(ParseMode::Html));
        let Some(InputPollMedia::Video(media)) = &poll.media else {
            panic!("expected video poll media");
        };
        assert_eq!(media.parse_mode, Some(ParseMode::Html));
    }

    #[test]
    fn send_poll_does_not_set_parse_modes_when_entities_are_explicit() {
        let entities = vec![];
        let mut poll = SendPoll::new(
            ChatId(1),
            "question",
            [InputPollOption::new("option").text_entities(entities.clone()).media(
                InputPollOptionMedia::Photo(
                    InputMediaPhoto::new(media()).caption_entities(entities.clone()),
                ),
            )],
        );
        poll.question_entities = Some(entities.clone());
        poll.explanation_entities = Some(entities.clone());
        poll.description_entities = Some(entities.clone());
        poll.explanation_media = Some(InputPollMedia::Audio(
            InputMediaAudio::new(media()).caption_entities(entities.clone()),
        ));
        poll.media =
            Some(InputPollMedia::Video(InputMediaVideo::new(media()).caption_entities(entities)));

        poll.visit_parse_modes(set_default_parse_mode);

        assert_eq!(poll.question_parse_mode, None);
        assert_eq!(poll.explanation_parse_mode, None);
        assert_eq!(poll.description_parse_mode, None);
        assert!(matches!(
            poll.options[0].formatting,
            Some(InputPollOptionFormatting::TextEntities(_))
        ));
        let Some(InputPollOptionMedia::Photo(option_media)) = &poll.options[0].media else {
            panic!("expected photo option media");
        };
        assert_eq!(option_media.parse_mode, None);
        let Some(InputPollMedia::Audio(explanation_media)) = &poll.explanation_media else {
            panic!("expected audio explanation media");
        };
        assert_eq!(explanation_media.parse_mode, None);
        let Some(InputPollMedia::Video(media)) = &poll.media else {
            panic!("expected video poll media");
        };
        assert_eq!(media.parse_mode, None);
    }
}
