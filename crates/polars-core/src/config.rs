use std::sync::OnceLock;

use crate::POOL;

// Formatting environment variables (typically referenced/set from the python-side Config object)
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_MAX_COLS: &str = "POLARS_FMT_MAX_COLS";
pub(crate) const FMT_MAX_ROWS: &str = "POLARS_FMT_MAX_ROWS";
pub(crate) const FMT_STR_LEN: &str = "POLARS_FMT_STR_LEN";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_CELL_ALIGNMENT: &str = "POLARS_FMT_TABLE_CELL_ALIGNMENT";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_CELL_NUMERIC_ALIGNMENT: &str = "POLARS_FMT_TABLE_CELL_NUMERIC_ALIGNMENT";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_DATAFRAME_SHAPE_BELOW: &str = "POLARS_FMT_TABLE_DATAFRAME_SHAPE_BELOW";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_FORMATTING: &str = "POLARS_FMT_TABLE_FORMATTING";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_HIDE_COLUMN_DATA_TYPES: &str = "POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_HIDE_COLUMN_NAMES: &str = "POLARS_FMT_TABLE_HIDE_COLUMN_NAMES";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_HIDE_COLUMN_SEPARATOR: &str = "POLARS_FMT_TABLE_HIDE_COLUMN_SEPARATOR";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION: &str =
    "POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_INLINE_COLUMN_DATA_TYPE: &str =
    "POLARS_FMT_TABLE_INLINE_COLUMN_DATA_TYPE";
#[cfg(any(feature = "fmt", feature = "fmt_no_tty"))]
pub(crate) const FMT_TABLE_ROUNDED_CORNERS: &str = "POLARS_FMT_TABLE_ROUNDED_CORNERS";
pub(crate) const FMT_TABLE_CELL_LIST_LEN: &str = "POLARS_FMT_TABLE_CELL_LIST_LEN";

pub fn verbose() -> bool {
    static VERBOSE: OnceLock<bool> = OnceLock::new();
    *VERBOSE.get_or_init(|| {
        std::env::var("POLARS_VERBOSE")
            .map(|s| s == "1")
            .unwrap_or(false)
    })
}

pub fn get_file_prefetch_size() -> usize {
    static FILE_PREFETCH_SIZE: OnceLock<usize> = OnceLock::new();
    *FILE_PREFETCH_SIZE.get_or_init(|| {
        std::env::var("POLARS_PREFETCH_SIZE")
            .map(|s| {
                s.parse::<usize>()
                    .expect("POLARS_PREFETCH_SIZE must be integer")
            })
            .unwrap_or_else(|_| std::cmp::max(POOL.current_num_threads() * 2, 16))
    })
}

pub fn get_rg_prefetch_size() -> usize {
    static ROW_GROUP_PREFETCH_SIZE: OnceLock<usize> = OnceLock::new();
    *ROW_GROUP_PREFETCH_SIZE.get_or_init(|| {
        std::env::var("POLARS_ROW_GROUP_PREFETCH_SIZE")
            .map(|s| {
                s.parse::<usize>()
                    .expect("POLARS_ROW_GROUP_PREFETCH_SIZE must be integer")
            })
            // Set it to something big, but not unlimited.
            .unwrap_or_else(|_| std::cmp::max(get_file_prefetch_size(), 128))
    })
}

pub fn force_async() -> bool {
    static FORCE_ASYNC: OnceLock<bool> = OnceLock::new();
    *FORCE_ASYNC.get_or_init(|| {
        std::env::var("POLARS_FORCE_ASYNC")
            .map(|value| value == "1")
            .unwrap_or(false)
    })
}
