
extern crate clap;
extern crate web_view;

use std::{io, process, fs};
use std::io::{Read, Write};
use clap::{App, ArgMatches, load_yaml};
use web_view::*;

macro_rules! err { ($e:expr) => ( {
    writeln!(&mut io::stderr(), "{}", $e).unwrap();
    process::exit(1);
} ) }

fn main() {
    let args = &get_args();
    web_view::builder()
        .title(args.value_of("title").unwrap())
        .content(Content::Html(&make_html(args)))
        .size(
            get_size("width", args),
            get_size("height", args)
        )
        .resizable(true)
        .debug(false)
        .user_data(0)
        .invoke_handler(|_, res| match res {
            _ => {
                println!("{}", res);
                process::exit(0);
            }
        })
        .run()
        .unwrap();
}

fn get_args() -> ArgMatches {
    let spec = load_yaml!("cli.yaml");
    App::from(spec).get_matches()
}

fn get_size(direction: &str, args: &ArgMatches) -> i32 {
    args.value_of(direction).unwrap()
        .parse()
        .unwrap_or_else(|e|err!(e))
}

fn get_md(args: &ArgMatches) -> String {
    let mut buf = Vec::new();
    if args.is_present("INPUT") {
        fs::File::open(args.value_of("INPUT").unwrap())
            .unwrap_or_else(|e|err!(e))
            .read_to_end(&mut buf)
            .unwrap();
    } else {
        io::stdin().read_to_end(&mut buf).unwrap();
    }
    String::from_utf8(buf)
        .unwrap_or_else(|e|err!(e))
}

fn get_css(args: &ArgMatches) -> String {
    if args.is_present("css") {
        args.values_of("css").unwrap()
            .map(|v| inline_asset("style", &get_a_css(v)))
            .collect()
    } else {
        inline_asset("style",
            &include_str!("default.css")
            .to_string()
        )
    }
}

fn get_a_css(file: &str) -> String {
    let mut buf = Vec::new();
    fs::File::open(file)
        .unwrap_or_else(|e|err!(e))
        .read_to_end(&mut buf).unwrap();
    String::from_utf8(buf)
        .unwrap_or_else(|e|err!(e))
}

fn make_html(args: &ArgMatches) -> String {
    let md = inline_div_with_id("content", &get_md(&args));
    let css = get_css(&args);
    let js =
        inline_asset("script", include_str!("markdown-it.min.js"))
        + &inline_asset("script", include_str!("markdown-it-input.js"))
        + &inline_asset("script", include_str!("formdial.js"));
    format!(r#"
        <html>
            <head><meta charset='utf-8'>
                {style}
                {script}
            </head>

            <body>
                {div}
                <p>
                    <button id="ok">submit</button>
                </p>
            </body>
        </html>
    "#, style=css, script=js, div=md)
}

fn inline_asset(tag: &str, text: &str) -> String {
    format!(r#"<{tag}>{}</{tag}>"#, text, tag=tag)
}

fn inline_div_with_id(id: &str, text: &str) -> String {
    format!(r#"<div id="{}">{}</div>"#, id, text)
}

