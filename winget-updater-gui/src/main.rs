use std::{thread, sync::mpsc::{channel, Sender, Receiver}};

use eframe::{run_native, App, egui::{CentralPanel, ScrollArea, Grid, Button, TopBottomPanel, Layout}, NativeOptions, epaint::{Vec2}};
use winget_updater_library::wud::{get_packages_to_update, WinPackage, update_package};

struct UpdaterApp {
    packages: Vec<UpdaterListElement>,
    is_updating: bool,
    is_refreshing: bool,
    sender: Sender<UpdaterMessage>,
    receiver: Receiver<UpdaterMessage>,
    sender_refresh: Sender<Vec<WinPackage>>,
    receiver_refresh: Receiver<Vec<WinPackage>>,
}

struct UpdaterListElement {
    checked: bool,
    status: UpdateStatus,
    package: WinPackage
}

struct UpdaterMessage {
    message: String,
    payload: String,
}

#[derive(PartialEq)]
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
    fn new(_cc: &eframe::CreationContext<'_>) -> UpdaterApp {
        let (send, receive) = channel();
        let (send_refresh, receive_refresh) = channel();
        
        let mut updater = UpdaterApp { 
            packages: Vec::new(),
            is_updating: false,
            is_refreshing: false,
            sender: send,
            receiver: receive,
            sender_refresh: send_refresh,
            receiver_refresh: receive_refresh
        };

        updater.refresh_package_list();

        updater
    }

    fn handle_package_grid(&mut self, ui: &mut eframe::egui::Ui) {
        ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            Grid::new("package_grid").show(ui, |ui| {
                ui.label("Update");
                ui.label("Package Name");
                ui.label("Package ID");
                ui.label("Installed");
                ui.label("Available");
                ui.label("Status");
                ui.end_row();

                for item in self.packages.iter_mut() {
                    ui.add_enabled_ui(item.status == UpdateStatus::NoOp, |ui| {
                        ui.checkbox(&mut item.checked, "");
                    });
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
        if ui.add_enabled(!self.is_refreshing && !self.is_updating, Button::new("Update selected")).clicked() {
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
                    sender_copy.send(UpdaterMessage {message: "Package_Updating".to_string(), payload: ele.clone()}).unwrap();
                    update_package(ele.as_str());
                    sender_copy.send(UpdaterMessage {message: "Package_Done".to_string(), payload: ele}).unwrap();
                }
                sender_copy.send(UpdaterMessage {message: "Finished".to_string(), payload: String::new()}).unwrap();
            });                
        } else if self.is_updating {
            let result = self.receiver.try_recv();
            match result {
                Ok(content) => {
                    match content.message.as_str() {
                        "Package_Updating" => {
                            let package = self.packages.iter_mut().find(|elem| elem.package.id.eq(&content.payload)).unwrap();
                            package.status = UpdateStatus::Updating;
                        }
                        "Package_Done" => {
                            let package = self.packages.iter_mut().find(|elem| elem.package.id.eq(&content.payload)).unwrap();
                            package.status = UpdateStatus::Done;
                            package.checked = false;
                        }
                        "Finished" => { self.is_updating = false }
                        _ => {}
                    }
                }
                Err(_) => {} // ignoring failed try receives..
            }
        }
    }

    fn refresh_package_list(&mut self) {
        self.is_refreshing = true;
        self.packages.clear();

        let sender_copy = self.sender_refresh.clone();

        thread::spawn(move|| {
            let updatable_packages = get_packages_to_update(Vec::new());
            sender_copy.send(updatable_packages).unwrap();
        });
        
    }

    fn handle_refresh_button(&mut self, ui: &mut eframe::egui::Ui) {
        if ui.add_enabled(!self.is_refreshing && !self.is_updating, Button::new("Refresh")).clicked() {
            self.refresh_package_list();
        } else if self.is_refreshing {
            let result = self.receiver_refresh.try_recv();
            match result {
                Ok(updatable_packages) => {
                    self.is_refreshing = false;
                    let ui_packages: Vec<UpdaterListElement> = updatable_packages.into_iter().map(|package| {
                        UpdaterListElement {
                            checked: false, 
                            status: UpdateStatus::NoOp,
                            package: package
                        }
                    } ).collect();
                    self.packages = ui_packages;
                }
                Err(_) => {}
            }
        }
    }
}

impl App for UpdaterApp {

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        CentralPanel::default().show(ctx, |ui| {
            self.handle_package_grid(ui);
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.handle_update_button(ui);
                ui.with_layout(Layout::right_to_left(eframe::emath::Align::Center), |ui| {
                    self.handle_refresh_button(ui);
                });
            });
        });
    }    
}

fn main() {
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(830f32, 600f32));
    run_native("WinGet Updater", win_options, Box::new(|cc| Box::new(UpdaterApp::new(cc))));
}
