use eframe::{run_native, epi::App, egui::{CentralPanel, Label, ScrollArea, Layout}, NativeOptions, epaint::Vec2};
use winget_updater_library::wud::{get_packages_to_update, update_package, WinPackage};

struct UpdaterApp {
    packages: Vec<WinPackage>
}

impl UpdaterApp {
    fn new() -> UpdaterApp {
        UpdaterApp { 
            packages: get_packages_to_update(Vec::new())
        }
    }
}

impl App for UpdaterApp {

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false, true]).show(ui, |ui| {
                for package in &self.packages {
                    ui.horizontal( |ui| {
                        ui.label(&package.name);
                        ui.label(&package.id);
                        ui.label(&package.installed_version);
                        ui.label(&package.available_version);
                    });
                    
                }
            });
            if ui.button("Update selected").clicked() {
                
            }
        });
    }

    fn name(&self) -> &str {
        "WinGet Updater"
    }
}

fn main() {
    let app = UpdaterApp::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(600f32, 400f32));
    run_native(Box::new(app), win_options);
}
