use std::thread;

use eframe::{run_native, epi::App, egui::{CentralPanel, ScrollArea, Grid, Button}, NativeOptions, epaint::Vec2};
use winget_updater_library::wud::{get_packages_to_update, WinPackage, update_package};

struct UpdaterApp {
    packages: Vec<(bool, WinPackage)>,
    is_updating: bool,
}

impl UpdaterApp {
    fn new() -> UpdaterApp {
        let updatable_packages = get_packages_to_update(Vec::new());
        let ui_packages = updatable_packages.into_iter().map(|package| (false, package) ).collect();
        UpdaterApp { 
            packages: ui_packages,
            is_updating: false,
        }
    }
}

impl App for UpdaterApp {

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false, true]).show(ui, |ui| {
                Grid::new("package_grid").show(ui, |ui| {
                    for (checked, package) in self.packages.iter_mut() {
                        ui.checkbox(checked, "|");
                        ui.label(&package.name);
                        ui.label(&package.id);
                        ui.label(&package.installed_version);
                        ui.label(&package.available_version);
                        ui.end_row();
                    }
                });
            });

            if ui.add_enabled(!self.is_updating, Button::new("Update selected")).clicked() {
                self.is_updating = true;
                
                /*let packages: Vec<_> = self.packages.iter().filter_map(|(enabled, package)| {
                    if *enabled { () }

                    Some(package.id.as_str())
                }).collect();


                let updater_handle = thread::spawn(move|| {
                    for ele in packages {
                        
                    }
                });*/
                
                for (requested_update, package) in self.packages.iter() {
                    if *requested_update {
                        update_package(&package.id.as_str());
                    }
                }
                self.is_updating = false;
                
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
