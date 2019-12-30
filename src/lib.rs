
pub mod file_remove_iterator;
pub mod remove_engine;
pub mod io_engine;

pub use remove_engine::{remove, remove_duplicates_files, Mode};
pub use file_remove_iterator::remove_duplicates;