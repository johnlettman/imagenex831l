#![allow(dead_code)]

use const_format::concatcp;
use git_version::git_version;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_VERSION: &str = git_version!(fallback = "unknown");
pub const IDENTIFIER: &str = concatcp!(VERSION, " ", GIT_VERSION);
