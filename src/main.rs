// With the default subsystem, 'console', windows creates an additional console window for the program.
// This is silently ignored on non-windows systems.
// See https://msdn.microsoft.com/en-us/library/4cc7ya5b.aspx for more details.
#![windows_subsystem = "windows"]

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::process;

use getopts::Options;
#[allow(unused_imports)]
use log::{debug, error, info, LevelFilter, trace, warn};
use simple_logger::SimpleLogger;
#[cfg(windows)]
use winapi::um::wincon::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole};

const SETTINGS_FILENAME: &str = "test_ui.toml";
const LOG_TARGET_MAIN: &str = "test_ui::Main";

#[cfg(feature = "webgui")]
mod web_ui;

fn main() {

    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    
    app.set_icon_from_file("webview/desktop.ico");
    
    app.add_menu_item("Quit", |window| {
        
        Ok::<_, systray::Error>(())
    });

    // When linked with the windows subsystem windows won't automatically attach
    // to the console of the parent process, so we do it explicitly. This fails silently if the parent has no console.
    #[cfg(windows)]
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("n", "nogui", "Run without graphic user interface (default for no gui builds)");
    opts.optflag("v", "verbose", "Show more debug messages");
    opts.optflag("d", "debug", "Show trace messages, more than debug");
    opts.optflag("l", "list", "List blocks from DB and exit");
    opts.optopt("c", "config", "Path to config file", "");

    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if opt_matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    let mut level = LevelFilter::Info;
    if opt_matches.opt_present("v") {
        level = LevelFilter::Debug;
    }
    if opt_matches.opt_present("d") {
        level = LevelFilter::Trace;
    }
    SimpleLogger::new()
        .with_level(level)
        .with_module_level("mio::poll", LevelFilter::Warn)
        .init()
        .unwrap();
    info!(target: LOG_TARGET_MAIN, "Starting DEMO {}", env!("CARGO_PKG_VERSION"));

    #[cfg(feature = "webgui")]
    web_ui::run_interface();

    // Without explicitly detaching the console cmd won't redraw it's prompt.
    #[cfg(windows)]
    unsafe {
        FreeConsole();
    }
}
