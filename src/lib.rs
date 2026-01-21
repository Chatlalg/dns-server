type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod buffer;
pub mod codes;
pub mod header;
pub mod query;
pub mod question;
pub mod record;
pub mod packet;
pub mod helpers;