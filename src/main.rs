use eframe::egui;
struct ImageConvertor {
    input_path: Option<String>,
}

impl ImageConvertor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input_path: None,
        }
    }
}

impl eframe::App for ImageConvertor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Convertor");
            ui.add_space(10.0);
            if ui.button("Select an Image to Convert").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_path = Some(path.display().to_string());
                }
            } 

            ui.label(format!("Selected Image: {:?}", self.input_path));

            if ui.button("Covert").clicked() {
                
            }
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