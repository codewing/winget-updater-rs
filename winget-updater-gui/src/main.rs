use std::{thread, sync::mpsc::{channel, Sender, Receiver}};

use eframe::{run_native, epi::{App}, egui::{CentralPanel, ScrollArea, Grid, Button, TopBottomPanel}, NativeOptions, epaint::{Vec2}};
use winget_updater_library::wud::{get_packages_to_update, WinPackage, update_package};

struct UpdaterApp {
    packages: Vec<UpdaterListElement>,
    is_updating: bool,
    sender: Sender<bool>,
    receiver: Receiver<bool>,
}

struct UpdaterListElement {
    checked: bool,
    status: UpdateStatus,
    package: WinPackage
}

enum UpdateStatus {
    NoOp,
    Waiting,
    Updating,
    Done
}

impl UpdaterListElement {
    fn status_message(&self) -> &str {
        match &self.status {
            UpdateStatus::NoOp => "",
            UpdateStatus::Waiting => "Waiting",
            UpdateStatus::Updating => "Updating",
            UpdateStatus::Done => "Done"
        }
    }
}

impl UpdaterApp {
    fn new() -> UpdaterApp {
        let updatable_packages = get_packages_to_update(Vec::new());
        let ui_packages = updatable_packages.into_iter().map(|package| {
            UpdaterListElement {
                checked: false, 
                status: UpdateStatus::NoOp,
                package: package
            }
        } ).collect();
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
                ui.label("Update");
                ui.label("Package Name");
                ui.label("Package ID");
                ui.label("Installed");
                ui.label("Available");
                ui.label("Status");
                ui.end_row();

                for item in self.packages.iter_mut() {
                    ui.checkbox(&mut item.checked, "|");
                    ui.label(&item.package.name);
                    ui.label(&item.package.id);
                    ui.label(&item.package.installed_version);
                    ui.label(&item.package.available_version);
                    ui.label(item.status_message());
                    ui.end_row();
                }
            });
        });
    }

    fn handle_update_button(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.add_enabled(!self.is_updating, Button::new("Update selected")).clicked() {
            self.is_updating = true;

            for element in &mut self.packages {
                if element.checked {
                    element.status = UpdateStatus::Waiting
                }
            }
            
            let packages: Vec<String> = self.packages.iter().filter_map(|item| {
                match item.checked {
                    true => Some(item.package.id.clone()),
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
