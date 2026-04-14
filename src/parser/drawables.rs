//! 扫描 drawable 目录的模块。

use log::{debug, info};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// 扫描 Drawables (提取路径和扩展名)
/// 
/// # 参数
/// - `res_dir`: 资源目录的路径
/// 
/// # 返回
/// (方法名, 绝对路径, 扩展名) 的向量
pub fn parse_drawables(res_dir: &Path) -> Vec<(String, String, String)> {
    let mut items = Vec::new();
    let dir = res_dir.join("drawable");

    debug!("Starting to parse drawables from: {}", dir.display());

    if dir.exists() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                let ext = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_string();
                if let Ok(abs_path) = fs::canonicalize(path) {
                    debug!("Found drawable: {} with ext {}", stem, ext);
                    items.push((stem, abs_path.to_str().unwrap().to_string(), ext));
                }
            }
        }
    } else {
        info!("drawable directory not found at: {}", dir.display());
    }

    info!("Successfully parsed {} drawables", items.len());
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_parse_drawables_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let drawables = parse_drawables(temp_dir.path());
        assert!(drawables.is_empty());
    }

    #[test]
    fn test_parse_drawables_with_files() {
        let temp_dir = tempdir().unwrap();
        let drawable_dir = temp_dir.path().join("drawable");
        fs::create_dir(&drawable_dir).unwrap();

        let png_file = drawable_dir.join("logo.png");
        fs::write(&png_file, "test png data").unwrap();

        let svg_file = drawable_dir.join("icon.svg");
        fs::write(&svg_file, "<svg></svg>").unwrap();

        let drawables = parse_drawables(temp_dir.path());
        assert_eq!(drawables.len(), 2);
        
        let names: Vec<&str> = drawables.iter().map(|(n, _, _)| n.as_str()).collect();
        assert!(names.contains(&"logo"));
        assert!(names.contains(&"icon"));

        let exts: Vec<&str> = drawables.iter().map(|(_, _, e)| e.as_str()).collect();
        assert!(exts.contains(&"png"));
        assert!(exts.contains(&"svg"));
    }
}
