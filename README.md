# resgen

一个用于 Rust GUI 应用的资源生成器，支持 Android 风格的资源管理。

## 功能特性

- **字符串资源**: 多语言支持，支持 `%s` 占位符转换为 `{}`
- **颜色资源**: 浅色/深色主题支持，支持十六进制颜色解析
- **尺寸资源**: 数值类型尺寸资源
- **可绘制资源**: 图片和 SVG 资源，使用 `OnceLock` 缓存
- **Android 风格的目录结构**: 如 `res/values/`, `res/values-night/` 等

## 使用方法

将 resgen 添加到你的 `Cargo.toml`:

```toml
[build-dependencies]
resgen = "0.1"
```

创建一个 `build.rs` 文件:

```rust
fn main() {
    resgen::Config::new()
        .res_dir("res")
        .build()
        .expect("资源生成失败");
}
```

在你的代码中包含生成的资源:

```rust
include!(concat!(env!("OUT_DIR"), "/r_generated.rs"));

use R::*;

fn main() {
    // 使用字符串资源
    strings::set_locale("en");
    let greeting = get_string(string::welcome);
    println!("{}", greeting);

    // 使用颜色资源
    let primary_color = color::primary(&iced::Theme::Light);
    
    // 使用尺寸资源
    let margin = dimen::margin;
    
    // 使用可绘制资源
    let logo = drawable::logo();
}
```

## 目录结构

按照 Android 风格组织你的资源:

```
res/
├── values/
│   ├── strings.xml
│   ├── colors.xml
│   └── dimens.xml
├── values-night/
│   └── colors.xml
├── values-zh-rCN/
│   └── strings.xml
└── drawable/
    ├── logo.png
    └── icon.svg
```

## 示例

查看 [examples](./examples) 目录获取完整的工作示例。

## 许可证

MIT
