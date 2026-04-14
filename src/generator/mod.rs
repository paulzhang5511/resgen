//! 核心代码生成器模块，负责将中间数据结构转换为 Rust 代码。

pub mod strings;
pub mod colors;
pub mod dimens;
pub mod drawables;
pub mod r_entry;

pub use strings::gen_strings;
pub use colors::gen_colors;
pub use dimens::gen_dimens;
pub use drawables::gen_drawables;
pub use r_entry::gen_r_entry;
