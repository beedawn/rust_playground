#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use chrono::{offset::TimeZone, DateTime, Local, NaiveDate};
use time::{Date, OffsetDateTime, UtcOffset, Duration, Time, Month};
use std::ops::Sub;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([360.0, 200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "countdown",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    now_offset: OffsetDateTime,
    now_date: NaiveDate,
    deadline: NaiveDate,
    countdown: chrono::Duration,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            now_offset: OffsetDateTime::now_utc(),
            now_date: NaiveDate::from_ymd_opt(OffsetDateTime::now_utc().year(),OffsetDateTime::now_utc().month() as u32,OffsetDateTime::now_utc().day() as u32).expect("today date error"),
            deadline: NaiveDate::from_ymd_opt(OffsetDateTime::now_utc().year(),OffsetDateTime::now_utc().month() as u32,OffsetDateTime::now_utc().day() as u32).expect("today date error"),
            countdown: chrono::Duration::seconds(0),


        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("countdown");

            ui.horizontal(|ui| {
             //   let name_label = ui.label("Your name: ");
              //  ui.text_edit_singleline(&mut self.now_date)
              //       .labelled_by(name_label.id);
            });
           // ui.label(format!("Today's date is '{}'", self.now_date));
            //ui.add(egui_extras::DatePickerButton::new(&mut self.now_date));
           // ui.add(egui_extras::DatePickerButton::new(&mut self.deadline));
            if ui.add(egui_extras::DatePickerButton::new(&mut self.deadline).arrows(false)).clicked_elsewhere(){
                println!("calendar clicker!");


                println!("The date is: {}", self.now_date);
                // println!("The date is: {}", utc.date());

                // let date = utc.date();

                // let dt = OffsetDateTime::new_in_offset(
                //     Date::from_calendar_date(2024, Month::December, 15).expect("mumma mia"),
                //     Time::from_hms_nano(12, 59, 59, 500_000_000).expect("seconds error"),
                //     UtcOffset::from_hms(-5, 0, 0).expect("timezone error"),
                // );

                let dt = self.deadline;
                println!("Calculated date is {}",dt);

                let diff = dt.sub(self.now_date);
                println!("{}", diff);
                println!("Weeks until deadline: {}", diff.num_weeks());
                println!("Days until deadline: {}", diff.num_days());
                println!("Hours until deadline: {}", diff.num_hours());
                println!("Seconds until deadline: {}", diff.num_seconds());

                self.countdown = diff.clone();
            }

            //ui.label(format!("Hello '{}', age {}", self.now_date, self.deadline));
            // if ui.button("Test").clicked(){
            //
            //
            //     let utc = self.now_date;
            //     println!("The date is: {}", utc);
            //     // println!("The date is: {}", utc.date());
            //
            //     // let date = utc.date();
            //
            //     // let dt = OffsetDateTime::new_in_offset(
            //     //     Date::from_calendar_date(2024, Month::December, 15).expect("mumma mia"),
            //     //     Time::from_hms_nano(12, 59, 59, 500_000_000).expect("seconds error"),
            //     //     UtcOffset::from_hms(-5, 0, 0).expect("timezone error"),
            //     // );
            //
            //     let dt = self.deadline;
            //     println!("Calculated date is {}",dt);
            //
            //     let diff = dt.sub(utc);
            //     println!("{}", diff);
            //     println!("Weeks until deadline: {}", diff.num_weeks());
            //     println!("Days until deadline: {}", diff.num_days());
            //     println!("Hours until deadline: {}", diff.num_hours());
            //     println!("Seconds until deadline: {}", diff.num_seconds());
            //
            //     self.countdown = diff.num_days().clone();
            //
            //     println!("Days until deadline minus: {}", diff.num_days());
            //     let diff_hours = diff.num_hours() -(24*(diff.num_days()));
            //     println!("Hours until deadline: {}", diff_hours);
            //     //let diff3 = diff2.sub(diff2.whole_hours());
            //     let diff_minutes = diff.num_minutes() - (60*diff.num_hours());
            //     println!("minutes: {}", diff_minutes);
            //
            //     let diff_seconds = diff.num_seconds() - (60*diff.num_minutes());
            //     println!("second until deadline: {}", diff_seconds);
            // };
            ui.label(format!("Now Time:{}:{}:{}",self.now_offset.hour(),self.now_offset.minute(),self.now_offset.second()));
            ui.label(format!("Days to date:{:?}",self.countdown.num_days()));
            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
        self.now_offset = OffsetDateTime::now_utc();
        self.countdown = self.now_offset.sub(self.deadline);
        ctx.request_repaint_after(core::time::Duration::from_secs(5));
    }


}


/*

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