use crate::graphics::render::shaders::Blob;

use std::{error, fmt, result};

use winapi::shared::winerror;
use winapi::um::winnt;

pub use Error::*;

pub type Result<T> = result::Result<T, Error>;

pub enum Okay {
    HResult(winnt::HRESULT),
}

#[derive(Debug)]
pub enum Error {
    Blob(Blob),
    HResult(winnt::HRESULT),
    NullPointer,
}

impl From<Blob> for Error {
    fn from(blob: Blob) -> Self {
        Blob(blob)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Blob(blob) => write!(f, "HRESULT: {}", blob),
            HResult(hresult) => write!(f, "HRESULT: {}", hresult),
            NullPointer => write!(f, "Null Pointer Encountered"),
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
