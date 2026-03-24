mod app;
mod mcp_install;

use app::TextViewerApp;
use eframe::egui;
use std::path::Path;

fn configure_fonts_windows(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    for (font_name, font_path) in [
        ("cjk-simhei", "C:\\Windows\\Fonts\\simhei.ttf"),
        ("cjk-deng", "C:\\Windows\\Fonts\\Deng.ttf"),
        ("cjk-simsunb", "C:\\Windows\\Fonts\\simsunb.ttf"),
    ] {
        if let Ok(bytes) = std::fs::read(Path::new(font_path)) {
            fonts.font_data.insert(
                font_name.to_owned(),
                egui::FontData::from_owned(bytes).into(),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, font_name.to_owned());
            ctx.set_fonts(fonts);
            return;
        }
    }
}

fn configure_fonts_linux(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 使用 include_bytes! 宏，直接读取项目根目录下的字体文件
    let font_data = include_bytes!("../font/SourceHanSansSC-Normal.otf"); // 黑体

    // 将字体数据插入 egui 字体库
    fonts.font_data.insert(
        "simhei".to_owned(),
        egui::FontData::from_static(font_data).into(),
    );

    // 将 "simhei" 设置为比例字体（Proportional）的第一优先级
    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        vec.insert(0, "simhei".to_owned());
    }

    // 将 "simhei" 设置为等宽字体（Monospace）的第一优先级
    if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        vec.insert(0, "simhei".to_owned());
    }

    // 生效配置
    ctx.set_fonts(fonts);
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Taint Rev Trace"),
        ..Default::default()
    };

    eframe::run_native(
        "Taint Rev Trace",
        options,
        Box::new(|cc| {
            configure_fonts_linux(&cc.egui_ctx);
            Ok(Box::new(TextViewerApp::default()))
        }),
    )
}
