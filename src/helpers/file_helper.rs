use dioxus::{html::FormData, prelude::Event};
use mime_guess::Mime;

#[derive(Debug, thiserror::Error)]
pub enum FileHelperError {
    #[error("Failure to get file engine from form event.")]
    NoFileEngineError,
    #[error("There was no file uploaded.")]
    NoFileError,
    #[error("Failure to read file data.")]
    FileDataError,
    #[error("Failure to detect MIME type.")]
    MimeTypeError,
}

pub async fn get_file_data(
    e: Event<FormData>,
    file_data: &mut Option<Vec<u8>>,
    file_type: &mut Option<Mime>,
) -> Result<(), FileHelperError> {
    if let Some(file_engine) = e.files() {
        if let Some(file) = file_engine.files().into_iter().next() {
            let data = file_engine.read_file(file.as_str()).await;
            if let Some(data) = data {
                if let Some(mime_type) = mime_guess::from_path(file).first() {
                    *file_data = Some(data);
                    *file_type = Some(mime_type);
                    Ok(())
                } else {
                    Err(FileHelperError::MimeTypeError)
                }
            } else {
                Err(FileHelperError::FileDataError)
            }
        } else {
            Err(FileHelperError::NoFileError)
        }
    } else {
        Err(FileHelperError::NoFileEngineError)
    }
}
