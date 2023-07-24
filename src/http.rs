#[cfg(feature = "blocking")]
pub mod blocking;


#[cfg(feature = "blocking")]
pub use blocking::HttpClient;
