// With the default subsystem, 'console', windows creates an additional console window for the program.
// This is silently ignored on non-windows systems.
// See https://msdn.microsoft.com/en-us/library/4cc7ya5b.aspx for more details.
#![windows_subsystem = "windows"]

extern crate web_view;
extern crate tinyfiledialogs as tfd;
extern crate serde;
extern crate serde_json;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use web_view::Content;
#[allow(unused_imports)]
use log::{debug, error, info, LevelFilter, trace, warn};
use serde::Deserialize;

use Cmd::*;
use self::web_view::WebView;

use std::env;
use std::thread;
use std::process;

use getopts::Options;
#[allow(unused_imports)]
use simple_logger::SimpleLogger;
#[cfg(windows)]
use winapi::um::wincon::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole};

const SETTINGS_FILENAME: &str = "test_ui.toml";
const LOG_TARGET_MAIN: &str = "test_ui::Main";

#[cfg(feature = "webgui")]
mod web_ui;

pub struct Context {
}

impl Context {
    /// Creating an essential context to work with
    pub fn new() -> Context {
        Context {  }
    }
}

fn main() -> Result<(), systray_ti::Error> {

    #[cfg(windows)]
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }

    let file_content = include_str!("webview/index.html");
//    let mut styles = inline_style(include_str!("webview/bulma.css"));
//    styles.push_str(&inline_style(include_str!("webview/styles.css")));
//    styles.push_str(&inline_style(include_str!("webview/busy_indicator.css")));
//    let scripts = inline_script(include_str!("webview/scripts.js"));

    let html = Content::Html(file_content);
    let title = format!("TEST {}", env!("CARGO_PKG_VERSION"));
    let context = Context::new();
    let context: Arc<Mutex<Context>> = Arc::new(Mutex::new(context));

    let mut interface = web_view::builder()
        .title(&title)
        .content(html)
        .size(550, 500)
        .min_size(420, 380)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|web_view, arg| {
            match serde_json::from_str(arg).unwrap() {
                SignIn => { sign_in(&context, web_view); }
            }
            debug!("Command {}", arg);
            Ok(())
        })
        .build()
	.unwrap();


    let mut app;
    match systray_ti::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file("webview/desktop.ico");
    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray_ti::Error>(())
    });

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray_ti::Error>(())
        });
        window.add_menu_separator();
        Ok::<_, systray_ti::Error>(())
    });

    app.add_menu_separator();

    app.add_menu_item("Quit", |window| {
        window.quit();
	process::exit(0x0100);
        Ok::<_, systray_ti::Error>(())
    });

    // When linked with the windows subsystem windows won't automatically attach
    // to the console of the parent process, so we do it explicitly. This fails silently if the parent has no console.

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
    };

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

    thread::spawn(move || {
      app.wait_for_message();
    });

    let pause = Duration::from_millis(25);
    let mut start = Instant::now();
    loop {
        match interface.step() {
            None => {
                info!("Interface closed, exiting");
                thread::sleep(Duration::from_millis(100));
                break;
            }
            Some(result) => {
                match result {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Something wrong with webview, exiting");
                        break;
                    }
                }
            }
        }
        if start.elapsed().as_millis() > 1 {
            thread::sleep(pause);
            start = Instant::now();
        }
    }
    // Without explicitly detaching the console cmd won't redraw it's prompt.
    #[cfg(windows)]
    unsafe {
        FreeConsole();
    };
    Ok(())
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    SignIn
}

fn sign_in(context: &Arc<Mutex<Context>>, web_view: &mut WebView<()>) {
    info!("Clicked Sign In button");
    web_view.eval("show_label();");
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
