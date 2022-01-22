use bevy::prelude::App;

use crate::{fuzz_runner, parse_commands};

use super::FuzzTarget;

pub fn bin_bootstrap(mut app_builder: impl FuzzTarget, mut args: std::env::Args) {
    args.next().unwrap(); // bin path

    let mode = args.next().unwrap_or(String::from("mode_not_found"));

    let mut app = App::new();

    match mode.as_str() {
        "record" => {
            println!("FUZZ: recording input events");
            app_builder.enable_recording_mode(&mut app);
            app.run();
        }
        "view" => {
            println!("FUZZ: print recorded input");
            let path = match args.next() {
                Some(path) => std::path::PathBuf::from(path),
                None => {
                    println!("\tplease supply the event .bin file as second argument");
                    return;
                }
            };

            let contents = match std::fs::read(&path) {
                Ok(c) => c,
                Err(e) => {
                    println!("\terror reading file {:?}: {:?}", path, e);
                    return;
                }
            };

            let data = match parse_commands(contents) {
                Ok(val) => val,
                Err(e) => {
                    println!("\terror parsing input commands: {:?}", e);
                    return;
                }
            };

            println!("FUZZ INPUT: {:#?}", data);
        }
        "gui" => {
            println!("FUZZ: running app in GUI mode");
            app_builder.enable_gui_mode(&mut app);
            app.run();
        }
        "apply" => {
            println!("FUZZ: applying input events");
            let path = match args.next() {
                Some(path) => std::path::PathBuf::from(path),
                None => {
                    println!("\tplease supply the event .bin file as second argument");
                    return;
                }
            };

            let contents = match std::fs::read(&path) {
                Ok(c) => c,
                Err(e) => {
                    println!("\terror reading file {:?}: {:?}", path, e);
                    return;
                }
            };

            let data = match parse_commands(contents) {
                Ok(val) => val,
                Err(e) => {
                    println!("\terror parsing input commands: {:?}", e);
                    return;
                }
            };

            app_builder.enable_apply_mode(&mut app, data);
            fuzz_runner(&mut app);
        }
        _ => {
            println!("Please use 'record', 'apply [filename]', 'view [filename]' or 'gui' as a parameter");
            return;
        }
    }
}
