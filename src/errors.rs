extern crate sdl2;
extern crate sdl2_ttf;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ScapeError {
    Str(String),
    Inner(Box<error::Error>),
    TextureValue(sdl2::render::TextureValueError),
}

impl fmt::Display for ScapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScapeError::Str(ref err) => write!(f, "{}", err),
            ScapeError::Inner(ref err) => write!(f, "{}", err),
            ScapeError::TextureValue(_) => write!(f, "TextureValueError"),
        }
    }
}

impl error::Error for ScapeError {
    fn description(&self) -> &str {
        match *self {
            ScapeError::Str(ref err) => err,
            ScapeError::Inner(ref err) => err.description(),
            ScapeError::TextureValue(_) => "TextureValueError",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ScapeError::Str(_) => None,
            ScapeError::Inner(ref err) => Some(err.as_ref()),
            ScapeError::TextureValue(_) => None,
        }
    }
}

impl From<String> for ScapeError {
    fn from(err: String) -> ScapeError {
        ScapeError::Str(err)
    }
}

impl From<&'static str> for ScapeError {
    fn from(err: &'static str) -> ScapeError {
        ScapeError::Str(err.to_owned())
    }
}

impl From<sdl2::video::WindowBuildError> for ScapeError {
    fn from(err: sdl2::video::WindowBuildError) -> ScapeError {
        ScapeError::Inner(Box::new(err))
    }
}

impl From<sdl2::IntegerOrSdlError> for ScapeError {
    fn from(err: sdl2::IntegerOrSdlError) -> ScapeError {
        ScapeError::Inner(Box::new(err))
    }
}

impl From<sdl2::render::TextureValueError> for ScapeError {
    fn from(err: sdl2::render::TextureValueError) -> ScapeError {
        ScapeError::TextureValue(err)
    }
}
