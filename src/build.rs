#![allow(dead_code)]

use const_format::concatcp;
use git_version::git_version;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const GIT_VERSION: &'static str = git_version!(fallback = "unknown");
pub const IDENTIFIER: &'static str = concatcp!(VERSION, " ", GIT_VERSION);
