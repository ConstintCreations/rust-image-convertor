use eframe::egui;
struct ImageConvertor {}

impl ImageConvertor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for ImageConvertor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Convertor!");
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Convertor",
         native_options,
        Box::new(|cc| Ok(Box::new(ImageConvertor::new(cc))))
    )
}