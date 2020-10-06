
use std::{io, process, fs};
use std::io::{Read, Write};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::BTreeMap;
use clap::{App, ArgMatches, load_yaml};
use serde_json::Value;
use convert_case::{Case, Casing};
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
                deser(args, res);
                process::exit(0);
            }
        })
        .run()
        .unwrap();
}

fn deser(args: &ArgMatches, res: &str) {
    let otype = args.value_of("output").unwrap()
        .to_case(Case::Lower);
    if "json" == otype {
        println!("{}", res);
    } else {
        let obj: BTreeMap<String, Value> =
            serde_json::from_str(res).unwrap();
        for (k, v) in obj.into_iter() {
            println!("{},{}", k, v);
        }
    }
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

fn make_html(args: &ArgMatches) -> String {
    let md = inline_div_with_id("content", &get_md(&args));
    let css = get_css(&args);
    let js =
        inline_asset("script", include_str!("markdown-it.min.js"))
        + &inline_asset("script", include_str!("markdown-it-input.js"))
        + &inline_asset("script", include_str!("formdial.js"));
    let submit = args.value_of("submit").unwrap();
    format!(r#"
    <html>
    <head> <meta charset='utf-8'>
        {style}
        {script}
    </head>

    <body>
        <form id="formdial-formtop">
            {div}
            <p>
                <button id="formdial-button-submit">
                    {label}
                </button>
            </p>
        </form>
    </body>
    </html>
    "#, style=css, script=js, div=md, label=submit)
}

fn inline_asset(tag: &str, text: &str) -> String {
    format!(r#"<{tag}>{}</{tag}>"#, text, tag=tag)
}

fn inline_div_with_id(id: &str, text: &str) -> String {
    format!(r#"<div id="{}">{}</div>"#, id, text)
}

fn get_md(args: &ArgMatches) -> String {
    let mut buf = Vec::new();
    if args.is_present("INPUT") {
        let _ = fs::File::open(args.value_of("INPUT").unwrap())
            .unwrap_or_else(|e|err!(e))
            .read_to_end(&mut buf);
    } else {
        let _ = io::stdin().read_to_end(&mut buf);
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
        let s = make_style_background(args)
            + "textarea:invalid,input:invalid { border: #f00 double };";
        inline_asset("style",
            &include_str!("default.css").to_string())
        + &inline_asset("style", &s)
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

fn make_style_background(args: &ArgMatches) -> String {
    if args.is_present("background") {
        let path = Path::new(args.value_of("background").unwrap());
        format!(
            "body {{ background:fixed url(data:image/{};base64,{}); }}",
            path.extension()
                .unwrap_or(OsStr::new(""))
                .to_str().unwrap(),
            get_image_base64(path)
        )
    } else { String::new() }
}

fn get_image_base64(path: &Path) -> String {
    let mut buf = Vec::new();
    let _ = fs::File::open(path)
        .unwrap_or_else(|e|err!(e))
        .read_to_end(&mut buf);
    base64::encode(buf)
}

