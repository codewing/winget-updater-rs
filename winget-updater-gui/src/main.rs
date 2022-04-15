use std::{thread, sync::mpsc::{channel, Sender, Receiver}};

use eframe::{run_native, epi::{App, IconData}, egui::{CentralPanel, ScrollArea, Grid, Button, TopBottomPanel}, NativeOptions, epaint::{Vec2}};
use winget_updater_library::wud::{get_packages_to_update, WinPackage, update_package};

struct UpdaterApp {
    packages: Vec<(bool, WinPackage)>,
    is_updating: bool,
    sender: Sender<bool>,
    receiver: Receiver<bool>,
}

impl UpdaterApp {
    fn new() -> UpdaterApp {
        let updatable_packages = get_packages_to_update(Vec::new());
        let ui_packages = updatable_packages.into_iter().map(|package| (false, package) ).collect();
        let (send, receive) = channel();

        UpdaterApp { 
            packages: ui_packages,
            is_updating: false,
            sender: send,
            receiver: receive,
        }
    }

    fn handle_package_grid(&mut self, ui: &mut eframe::egui::Ui) {
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
    }

    fn handle_update_button(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.add_enabled(!self.is_updating, Button::new("Update selected")).clicked() {
            self.is_updating = true;
            
            let packages: Vec<String> = self.packages.iter().filter_map(|(enabled, package)| {
                match *enabled {
                    true => Some(package.id.clone()),
                    false => None
                }
            }).collect();

            let sender_copy = self.sender.clone();

            thread::spawn(move|| {
                for ele in packages {
                    update_package(ele.as_str())
                }
                sender_copy.send(true).unwrap();
            });                
        } else if self.is_updating {
            let result = self.receiver.try_recv();
            match result {
                Ok(_) => {self.is_updating = false},
                Err(_) => {},
            }
        }
    }
}

impl App for UpdaterApp {

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        
        CentralPanel::default().show(ctx, |ui| {
            self.handle_package_grid(ui);
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.handle_update_button(ui);
        });
    }

    fn name(&self) -> &str {
        "WinGet Updater"
    }

    
}

fn main() {
    let app = UpdaterApp::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(800f32, 600f32));
    run_native(Box::new(app), win_options);
}
