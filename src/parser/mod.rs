//! 负责从 Android 目录结构中提取原始数据的模块。

pub mod strings;
pub mod colors;
pub mod dimens;
pub mod drawables;
pub mod utils;

pub use strings::{parse_strings, ParsedStrings};
pub use colors::parse_colors;
pub use dimens::parse_dimens;
pub use drawables::parse_drawables;
pub use utils::parse_hex_color;
