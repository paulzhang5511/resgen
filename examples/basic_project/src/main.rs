// 包含 resgen 生成的资源入口文件
include!("generated/r_generated.rs");

use R::*;

fn main() {
    // ============================
    // 使用字符串资源
    // ============================
    strings::set_locale("default");
    let app_name = get_string(string::app_name);
    let welcome = get_string(string::welcome);
    let description = get_string(string::description);
    println!("应用名称: {}", app_name);
    println!("欢迎消息: {}", welcome);
    println!("描述: {}", description);

    // ============================
    // 使用颜色资源
    // ============================
    let light_primary = color::primary(&iced::Theme::Light);
    let dark_primary = color::primary(&iced::Theme::Dark);
    println!("浅色主题主颜色: {:?}", light_primary);
    println!("深色主题主颜色: {:?}", dark_primary);

    // ============================
    // 使用尺寸资源
    // ============================
    println!("Margin: {}", dimen::margin);
    println!("Padding: {}", dimen::padding);
    println!("Text size: {}", dimen::text_size);
}
