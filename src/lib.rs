#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use server::KvsServer;
pub use thread_pool::ThreadPool;
pub use thread_pool::RayonThreadPool;

mod client;
mod common;
mod engines;
mod error;
mod server;
mod thread_pool;
