#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use time::{Date, OffsetDateTime, UtcOffset, Duration, Time, Month};
use std::ops::Sub;
/*
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Countdown",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!(
                "../../../crates/egui/assets/ferris.png"
            ));
        });
    }
}




fn main() {
    let utc = OffsetDateTime::now_utc();
    println!("The time is: {}", utc.time());
    println!("The date is: {}", utc.date());

   // let date = utc.date();

    let dt = OffsetDateTime::new_in_offset(
    Date::from_calendar_date(2024, Month::December, 15).expect("mumma mia"),
    Time::from_hms_nano(12, 59, 59, 500_000_000).expect("seconds error"),
    UtcOffset::from_hms(-5, 0, 0).expect("timezone error"),
);
    println!("Calculated date is {}",dt.date());

    let diff = dt.sub(utc);
    println!("{}", diff);
    println!("Weeks until deadline: {}", diff.whole_weeks()); 
    println!("Days until deadline: {}", diff.whole_days());
    println!("Hours until deadline: {}", diff.whole_hours());
    println!("Seconds until deadline: {}", diff.whole_seconds());



    println!("Days until deadline minus: {}", diff.whole_days());
    let diff_hours = diff.whole_hours() -(24*(diff.whole_days()));
    println!("Hours until deadline: {}", diff_hours);
    //let diff3 = diff2.sub(diff2.whole_hours());
    let diff_minutes = diff.whole_minutes() - (60*diff.whole_hours());
    println!("minutes: {}", diff_minutes);

    let diff_seconds = diff.whole_seconds() - (60*diff.whole_minutes());
    println!("second until deadline: {}", diff_seconds);
}

*/