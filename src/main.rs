use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use image::{DynamicImage, ImageFormat};


struct ImageConvertor {
    input_path: Option<PathBuf>,
    output_format: ImageFormat,
    converted_image: Option<DynamicImage>
}

impl ImageConvertor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            input_path: None,
            output_format: ImageFormat::Png,
            converted_image: None
        }
    }
}

impl eframe::App for ImageConvertor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Convertor");
            ui.add_space(10.0);
            if ui.button("Select an Image to Convert").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["bmp", "exr", "ff", "hdr", "pnm", "qoi", "tga", "ico", "jpeg", "png", "gif", "tiff"])
                    .pick_file()
                {
                    self.input_path = Some(path);
                    self.converted_image = None;
                }
            } 

            if let Some(path) = &self.input_path {
                ui.label(format!("Selected Image: {}", path.display()));
            } else {
                ui.label("No Image Selected");
            }

            egui::ComboBox::from_label("Output Format")
            .selected_text(format!("{:?}", self.output_format))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.output_format, ImageFormat::Png, "PNG");
                ui.selectable_value(&mut self.output_format, ImageFormat::Jpeg, "JPG");
                ui.selectable_value(&mut self.output_format, ImageFormat::Ico, "ICO");
                ui.selectable_value(&mut self.output_format, ImageFormat::Gif, "GIF");
                ui.selectable_value(&mut self.output_format, ImageFormat::Bmp, "BMP");
                ui.selectable_value(&mut self.output_format, ImageFormat::Tiff, "TIFF");

                ui.selectable_value(&mut self.output_format, ImageFormat::Hdr, "HDR");
                ui.selectable_value(&mut self.output_format, ImageFormat::Tga, "TGA");
                ui.selectable_value(&mut self.output_format, ImageFormat::Pnm, "PNM");
                ui.selectable_value(&mut self.output_format, ImageFormat::OpenExr, "EXR");
                ui.selectable_value(&mut self.output_format, ImageFormat::Farbfeld, "FF");
                ui.selectable_value(&mut self.output_format, ImageFormat::Qoi, "QOI");
            });

            if ui.button("Covert").clicked() {
                if let Some(path) = &self.input_path {
                    if let Ok(img) = image::open(path) {
                        self.converted_image = Some(img);
                    }
                }
            }

            if ui.button("Download").clicked() {
                if let Some(save_path) = FileDialog::new().save_file() {
                    if let Some(img) = &self.converted_image {
                        let _ = img.save_with_format(save_path, self.output_format);
                    }
                }
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