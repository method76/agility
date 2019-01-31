use snow::SnowError;
#[derive(Fail, Debug)]
pub enum NoiseError {
    #[fail(display = "Wrong handshake message length {}", _0)]
    WrongMessageLength(usize),

    #[fail(display = "{}", _0)]
    Other(String),

    #[fail(display = "Snow error: {}", _0)]
    Snow(SnowError),
}

impl From<SnowError> for NoiseError {
    fn from(err: SnowError) -> NoiseError {
        NoiseError::Snow(err)
    }
}
