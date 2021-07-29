//! Record program
#![deny(missing_docs)]

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current SDK types for downstream users building with a different SDK version
pub use solana_program;

solana_program::declare_id!("5hbY7FCW78iUZKpetXwYuN4roW7eMLinziA9MwS8CefL");
