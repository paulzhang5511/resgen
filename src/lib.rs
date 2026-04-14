//! # resgen
//!
//! A resource generator for Rust GUI applications (supports Android-style resource management)
use log::{debug, info};
use std::env;
use std::path::{Path, PathBuf};

pub mod generator;
pub mod parser;

pub use parser::{
    parse_strings, parse_colors, parse_dimens, parse_drawables,
    ParsedStrings,
};

pub use generator::{
    gen_strings, gen_colors, gen_dimens, gen_drawables, gen_r_entry,
};

/// 主配置结构体，用于配置资源生成过程
#[derive(Debug, Clone)]
pub struct Config {
    /// 资源目录路径
    res_dir: PathBuf,
    /// 输出目录路径
    out_dir: PathBuf,
}

impl Config {
    /// 创建一个新的 Config 实例
    ///
    /// 默认使用 CARGO_MANIFEST_DIR 下的 res 目录作为资源目录，
    /// 使用 OUT_DIR 作为输出目录
    pub fn new() -> Self {
        debug!("Creating new Config instance");
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        Self {
            res_dir: Path::new(&manifest_dir).join("res"),
            out_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
        }
    }

    /// 设置资源目录
    ///
    /// # 参数
    /// - `path`: 资源目录的路径
    ///
    /// # 返回
    /// 返回 Config 实例本身以支持链式调用
    pub fn res_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        debug!("Setting resource directory to: {}", path.as_ref().display());
        self.res_dir = path.as_ref().to_path_buf();
        self
    }

    /// 执行资源生成逻辑
    ///
    /// # 返回
    /// 成功返回 Ok(())，失败返回错误
    pub fn build(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting resource generation process");
        debug!("Resource directory: {}", self.res_dir.display());
        debug!("Output directory: {}", self.out_dir.display());

        let out_path = &self.out_dir;

        println!("cargo:rerun-if-changed={}", self.res_dir.display());

        info!("Parsing strings resources...");
        let strings = parser::parse_strings(&self.res_dir)?;

        info!("Parsing dimens resources...");
        let dimens = parser::parse_dimens(&self.res_dir);

        info!("Parsing drawables resources...");
        let drawables = parser::parse_drawables(&self.res_dir);

        info!("Parsing colors resources...");
        let light_colors = parser::parse_colors(&self.res_dir, false);
        let dark_colors = parser::parse_colors(&self.res_dir, true);

        let mut color_keys: Vec<String> = light_colors.keys().cloned().collect();
        for k in dark_colors.keys() {
            if !color_keys.contains(k) {
                color_keys.push(k.clone());
            }
        }
        color_keys.sort();

        info!("Generating code...");
        let string_code = generator::gen_strings(&strings.keys, &strings.locales, &strings.data);
        let color_code = generator::gen_colors(&color_keys, &light_colors, &dark_colors);
        let dimen_code = generator::gen_dimens(&dimens);
        let drawable_code = generator::gen_drawables(&drawables);
        let r_entry_code = generator::gen_r_entry();

        debug!("Writing generated files to output directory");
        std::fs::write(out_path.join("strings_generated.rs"), string_code)?;
        std::fs::write(out_path.join("colors_generated.rs"), color_code)?;
        std::fs::write(out_path.join("dimens_generated.rs"), dimen_code)?;
        std::fs::write(out_path.join("drawable_generated.rs"), drawable_code)?;
        std::fs::write(out_path.join("r_generated.rs"), r_entry_code)?;

        info!("Resource generation completed successfully");
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_debug() {
        // Just test that Config can be instantiated with custom paths
        let res_dir = PathBuf::from("/tmp/res");
        let out_dir = PathBuf::from("/tmp/out");
        
        // We can't test Config::new() directly without CARGO_MANIFEST_DIR and OUT_DIR
        // But we can test that Config has the right fields
        let config = Config {
            res_dir,
            out_dir,
        };
        assert_eq!(format!("{:?}", config).contains("/tmp/res"), true);
        assert_eq!(format!("{:?}", config).contains("/tmp/out"), true);
    }
}
