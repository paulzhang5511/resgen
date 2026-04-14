# resgen

一个用于 Rust GUI 应用的资源生成器，支持 Android 风格的资源管理。

## 功能特性

- **字符串资源**: 多语言支持，支持 `%s` 占位符转换为 `{}`
- **颜色资源**: 浅色/深色主题支持，支持十六进制颜色解析
- **尺寸资源**: 数值类型尺寸资源
- **可绘制资源**: 图片和 SVG 资源，使用 `OnceLock` 缓存
- **Android 风格的目录结构**: 如 `res/values/`, `res/values-night/` 等

## 生成的文件

构建后，resgen 会在 `OUT_DIR`（由 Cargo 自动设置）中生成以下文件：

| 文件名 | 说明 |
|--------|------|
| `strings_generated.rs` | 字符串资源代码（StringId 枚举、get_raw_string 函数等） |
| `colors_generated.rs` | 颜色资源代码（raw 模块、主题感知的颜色函数） |
| `dimens_generated.rs` | 尺寸资源代码（常量定义） |
| `drawable_generated.rs` | 可绘制资源代码（带 OnceLock 缓存的图片/ SVG 加载函数） |
| `r_generated.rs` | 总入口文件（整合了所有资源，提供 R 模块） |

## 使用方法

### 1. 添加依赖

将 resgen 添加到你的 `Cargo.toml`:

```toml
[build-dependencies]
resgen = "0.1"
```

### 2. 配置 build.rs

在项目根目录下创建一个 `build.rs` 文件:

```rust
fn main() {
    // 创建 Config 实例
    resgen::Config::new()
        // 指定资源目录（默认为 CARGO_MANIFEST_DIR 下的 "res" 目录）
        .res_dir("res")
        // 执行资源生成
        .build()
        .expect("资源生成失败");
}
```

### 3. 在代码中包含资源

在你的主要代码（通常是 `src/main.rs` 或 `src/lib.rs`）中包含生成的资源文件:

```rust
// 包含总入口文件
include!(concat!(env!("OUT_DIR"), "/r_generated.rs"));

// 引入 R 模块以便于使用
use R::*;

fn main() {
    // ============================
    // 使用字符串资源
    // ============================
    // 设置当前语言环境
    strings::set_locale("zh-rCN");
    // 获取字符串
    let app_name = get_string(string::app_name);
    let welcome_msg = get_string(string::welcome);
    println!("应用名称: {}", app_name);
    println!("欢迎消息: {}", welcome_msg);

    // ============================
    // 使用颜色资源
    // ============================
    // 获取浅色主题的颜色
    let light_primary = color::primary(&iced::Theme::Light);
    // 获取深色主题的颜色
    let dark_primary = color::primary(&iced::Theme::Dark);
    println!("浅色主题主颜色: {:?}", light_primary);
    println!("深色主题主颜色: {:?}", dark_primary);

    // ============================
    // 使用尺寸资源
    // ============================
    let margin = dimen::margin;
    let padding = dimen::padding;
    let text_size = dimen::text_size;
    println!("Margin: {}", margin);
    println!("Padding: {}", padding);
    println!("Text size: {}", text_size);

    // ============================
    // 使用可绘制资源
    // ============================
    // 加载图片（自动缓存）
    let logo = drawable::logo();
    let icon = drawable::icon();
    // logo 和 icon 可以直接在 UI 库中使用
}
```

## 目录结构

按照 Android 风格组织你的资源:

```
your_project/
├── Cargo.toml
├── build.rs
├── src/
│   └── main.rs
└── res/
    ├── values/
    │   ├── strings.xml      # 默认语言的字符串资源
    │   ├── colors.xml       # 浅色主题的颜色资源
    │   └── dimens.xml       # 尺寸资源
    ├── values-night/
    │   └── colors.xml       # 深色主题的颜色资源
    ├── values-zh-rCN/
    │   └── strings.xml      # 简体中文语言的字符串资源
    └── drawable/
        ├── logo.png         # 图片资源
        └── icon.svg         # SVG 资源
```

## 资源文件格式

### strings.xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">我的应用</string>
    <string name="welcome">你好，%s！</string>
    <string name="description">这是一个测试应用。</string>
</resources>
```

### colors.xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="primary">#3498db</color>
    <color name="secondary">#2ecc71</color>
    <color name="background">#ffffff</color>
    <color name="text">#333333</color>
</resources>
```

### dimens.xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <dimen name="margin">16.0</dimen>
    <dimen name="padding">8.0</dimen>
    <dimen name="text_size">14.0</dimen>
</resources>
```

## 示例

查看 [examples](./examples) 目录获取完整的工作示例。

## 许可证

MIT
