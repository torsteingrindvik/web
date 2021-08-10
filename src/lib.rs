#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// HTML functionality.
pub mod html;

/// The web server functionality.
pub mod server;

/// Open REST APIs used to display content on demand.
pub mod apis;

/// Mixed basket of utilities.
pub mod util;
