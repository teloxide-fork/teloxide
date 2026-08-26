use crate::{
    payloads,
    requests::Payload,
    types::{
        InputFile, InputFileLike, InputMedia, InputPaidMedia, InputPollMedia, InputPollOptionMedia,
        InputSticker,
    },
};

/// Payloads that need to be sent as `multipart/form-data` because they contain
/// files inside.
pub trait MultipartPayload: Payload {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile));

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile));
}

impl MultipartPayload for payloads::SendPaidMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputPaidMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputPaidMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::SendPoll {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media
            .iter()
            .flat_map(InputPollMedia::files)
            .chain(self.explanation_media.iter().flat_map(InputPollMedia::files))
            .chain(
                self.options
                    .iter()
                    .filter_map(|option| option.media.as_ref())
                    .flat_map(InputPollOptionMedia::files),
            )
            .for_each(|file| file.copy_into(into));
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media
            .iter_mut()
            .flat_map(InputPollMedia::files_mut)
            .chain(self.explanation_media.iter_mut().flat_map(InputPollMedia::files_mut))
            .chain(
                self.options
                    .iter_mut()
                    .filter_map(|option| option.media.as_mut())
                    .flat_map(InputPollOptionMedia::files_mut),
            )
            .for_each(|file| file.move_into(into));
    }
}

impl MultipartPayload for payloads::SendMediaGroup {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter().flat_map(InputMedia::files).for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.iter_mut().flat_map(InputMedia::files_mut).for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMedia {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::EditMessageMediaInline {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.media.files().for_each(|f| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.media.files_mut().for_each(|f| f.move_into(into))
    }
}

impl MultipartPayload for payloads::CreateNewStickerSet {
    fn copy_files(&self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter()
            .for_each(|InputSticker { sticker: f, .. }: &InputSticker| f.copy_into(into))
    }

    fn move_files(&mut self, into: &mut dyn FnMut(InputFile)) {
        self.stickers
            .iter_mut()
            .for_each(|InputSticker { sticker: f, .. }: &mut InputSticker| f.move_into(into))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatId, InputMediaPhoto, InputPollOption, InputPollOptionMedia};

    #[test]
    fn send_poll_collects_and_moves_nested_media_files() {
        let media_file = InputFile::memory("poll media");
        let explanation_media_file = InputFile::memory("explanation media");
        let option_media_file = InputFile::memory("option media");
        let expected_files = [&media_file, &explanation_media_file, &option_media_file]
            .map(|file| serde_json::to_string(file).unwrap());

        let mut payload = payloads::SendPoll::new(
            ChatId(0),
            "Question",
            [InputPollOption::new("Option")
                .media(InputPollOptionMedia::Photo(InputMediaPhoto::new(option_media_file)))],
        );
        payload.media = Some(InputPollMedia::Photo(InputMediaPhoto::new(media_file)));
        payload.explanation_media =
            Some(InputPollMedia::Photo(InputMediaPhoto::new(explanation_media_file)));

        let mut copied_files = Vec::new();
        payload.copy_files(&mut |file| copied_files.push(serde_json::to_string(&file).unwrap()));
        assert_eq!(copied_files, expected_files);

        let mut moved_files = Vec::new();
        payload.move_files(&mut |file| moved_files.push(serde_json::to_string(&file).unwrap()));
        assert_eq!(moved_files, expected_files);

        let serialized = serde_json::to_value(payload).unwrap();
        assert_eq!(serialized["media"]["media"], "");
        assert_eq!(serialized["explanation_media"]["media"], "");
        assert_eq!(serialized["options"][0]["media"]["media"], "");
    }
}
