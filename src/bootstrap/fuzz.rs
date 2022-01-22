use bevy::prelude::App;

use crate::{fuzz_runner, parse_commands, prelude::FuzzData};

use super::FuzzTarget;
use std::sync::Once;

static mut APP: Option<App> = None;
static INIT: Once = Once::new();

pub fn fuzz_bootstrap(mut app_builder: impl FuzzTarget, fuzz_bytes: &[u8]) {
    // if no fuzz bytes at all, do not continue
    if fuzz_bytes.len() == 0 {
        return;
    }

    // try to parse the commands from the input
    let fuzz_inputs = match parse_commands(fuzz_bytes.to_vec()) {
        Ok(d) => d,
        Err(_) => return,
    };

    // no commands at all parsed (garbage), do not continue
    if fuzz_inputs.len() == 0 {
        return;
    }

    // run the app
    unsafe {
        INIT.call_once(|| {
            // this is the app initialization, will be done only once
            // rationale: performance, the app init takes tens of milliseconds
            let mut app = App::new();
            app_builder.enable_fuzzing_mode(&mut app);
            APP = Some(app);
        });

        // get ref to app
        let app = APP.as_mut().unwrap();

        // feed the fuzz inputs
        app.insert_resource(FuzzData::new(fuzz_inputs));

        // run fuzz iteration
        fuzz_runner(app);
    }
}
