use thiserror::Error;

#[derive(Error, Debug)]
#[error("...")]
pub struct LargeError {
    a: [u8; 2048],
}

#[derive(Error, Debug)]
#[error("...")]
pub enum Autoboxed {
    Large(
        #[from]
        #[boxing]
        LargeError,
    ),
}

#[derive(Error, Debug)]
#[error("...")]
pub struct Autoboxed2 {
    #[from]
    #[boxing]
    err: LargeError,
}

#[derive(Error, Debug)]
#[error("...")]
pub struct Autoboxed3 {
    #[boxing]
    #[from]
    err: LargeError,
}

pub fn direct_return_large() -> Result<(), LargeError> {
    Err(LargeError { a: [0; 2048] })
}

pub fn autobox() -> Result<(), Box<Autoboxed>> {
    let _ = direct_return_large()?;

    Ok(())
}

pub fn autobox2() -> Result<(), Box<Autoboxed2>> {
    let _ = direct_return_large()?;

    Ok(())
}

pub fn autobox3() -> Result<(), Box<Autoboxed3>> {
    let _ = direct_return_large()?;

    Ok(())
}
