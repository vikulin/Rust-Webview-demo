extern crate web_view;
extern crate tinyfiledialogs as tfd;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use web_view::Content;
#[allow(unused_imports)]
use log::{debug, error, info, LevelFilter, trace, warn};
use serde::Deserialize;

use Cmd::*;
use self::web_view::WebView;

pub fn run_interface() {
    let file_content = include_str!("webview/index.html");
    let mut styles = inline_style(include_str!("webview/bulma.css"));
    styles.push_str(&inline_style(include_str!("webview/styles.css")));
    styles.push_str(&inline_style(include_str!("webview/busy_indicator.css")));
    //let scripts = inline_script(include_str!("webview/scripts.js"));

    let html = Content::Html(file_content.to_owned().replace("{styles}", &styles).replace("{scripts}", &scripts));
    let title = format!("TEST {}", env!("CARGO_PKG_VERSION"));
    let mut interface = web_view::builder()
        .title(&title)
        .content(html)
        .size(580, 450)
//        .min_size(420, 380)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|web_view, arg| {
            debug!("Command {}", arg);
            Ok(())
        })
        .build()
        .expect("Error building GUI");

    run_interface_loop(&mut interface);
    interface.exit();
}

/// Indefinitely loops through WebView steps
fn run_interface_loop(interface: &mut WebView<()>) {
    // We use this ugly loop to lower CPU usage a lot.
    // If we use .run() or only .step() in a loop without sleeps it will try
    // to support 60FPS and uses more CPU than it should.
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
}


#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
