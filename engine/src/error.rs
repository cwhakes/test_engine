use crate::graphics::render::shaders::Blob;

use std::{error, fmt, result};

use std::io;
use image::error::ImageError;
use winapi::shared::winerror;
use winapi::um::winnt;

pub use Error::*;

pub type Result<T> = result::Result<T, Error>;

pub enum Okay {
    HResult(winnt::HRESULT),
}

pub enum Error {
    Blob(Blob),
    HResult(winnt::HRESULT),
    ImageError(ImageError),
    Io(io::Error),
    NullPointer(&'static str, u32, u32),
}

impl From<Blob> for Error {
    fn from(blob: Blob) -> Self {
        Blob(blob)
    }
}

impl From<ImageError> for Error {
    fn from(image_err: ImageError) -> Self {
        Error::ImageError(image_err)
    }
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Error::Io(io_err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blob(blob) => write!(f, "Blob Error: {}", blob),
            HResult(hresult) => write!(f, "HRESULT: {:x}", hresult),
            ImageError(image_err) => write!(f, "Image Error: {}", image_err),
            Io(io_err) => write!(f, "Io Error: {}", io_err),
            NullPointer(file, line, col) => write!(f, "Null Pointer Encountered\nFile:{}\nLine:{} Column:{}", file, line, col),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blob(blob) => write!(f, "Blob Error: {}", blob),
            HResult(hresult) => write!(f, "HRESULT: {:x}", hresult),
            ImageError(image_err) => write!(f, "Image Error: {}", image_err),
            Io(io_err) => write!(f, "Io Error: {}", io_err),
            NullPointer(file, line, col) => write!(f, "Null Pointer Encountered\nFile:{}\nLine:{} Column:{}", file, line, col),
        }
    }
}

impl error::Error for Error{}

pub trait HResultToResult{
    fn result(self) -> Result<Okay>;
}

impl HResultToResult for winnt::HRESULT {
    fn result(self) -> Result<Okay> {
        if winerror::SUCCEEDED(self) {
            Ok(Okay::HResult(self))
        } else {
            Err(Error::HResult(self))
        }
    }
}

#[macro_export]
macro_rules! null_ptr_err {
    () => {
        crate::error::Error::NullPointer(file!(), line!(), column!())
    };
}
