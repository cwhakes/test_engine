use crate::graphics::resource::shader::Blob;

use std::{error, fmt, result};

use image::error::ImageError;
use std::io;
use wavefront_obj::ParseError;
use winapi::shared::winerror;
use winapi::um::winnt;

pub use Error::*;

pub type Result<T> = result::Result<T, Error>;

pub enum Okay {
    HResult(winnt::HRESULT),
}

pub enum Error {
    Blob(Blob),
    Custom(String),
    HResult(winnt::HRESULT),
    ImageError(ImageError),
    Io(io::Error),
    ObjError(ParseError),
    NullPointer(&'static str, u32, u32),
}

impl From<Blob> for Error {
    fn from(blob: Blob) -> Self {
        Blob(blob)
    }
}

impl From<ImageError> for Error {
    fn from(image_err: ImageError) -> Self {
        Self::ImageError(image_err)
    }
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Self::Io(io_err)
    }
}

impl From<ParseError> for Error {
    fn from(obj_error: ParseError) -> Self {
        Self::ObjError(obj_error)
    }
}

impl From<&str> for Error {
    fn from(text: &str) -> Self {
        Self::Custom(text.to_owned())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blob(blob) => write!(f, "Blob Error: {:?}", blob),
            Custom(string) => write!(f, "Custom Error: {:?}", string),
            HResult(hresult) => write!(f, "HRESULT: {:x}", hresult),
            ImageError(image_err) => write!(f, "Image Error: {:?}", image_err),
            Io(io_err) => write!(f, "Io Error: {:?}", io_err),
            ObjError(obj_err) => write!(f, "Obj Error: {:?}", obj_err),
            NullPointer(file, line, col) => write!(
                f,
                "Null Pointer Encountered\nFile:{}\nLine:{} Column:{}",
                file, line, col
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blob(blob) => write!(f, "Blob Error: {}", blob),
            Custom(string) => write!(f, "Custom Error: {:?}", string),
            HResult(hresult) => write!(f, "HRESULT: {:x}", hresult),
            ImageError(image_err) => write!(f, "Image Error: {}", image_err),
            Io(io_err) => write!(f, "Io Error: {}", io_err),
            ObjError(obj_err) => write!(f, "Obj Error: {:?}", obj_err),
            NullPointer(file, line, col) => write!(
                f,
                "Null Pointer Encountered\nFile:{}\nLine:{} Column:{}",
                file, line, col
            ),
        }
    }
}

impl error::Error for Error {}

pub trait HResultToResult {
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
        $crate::error::Error::NullPointer(file!(), line!(), column!())
    };
}
