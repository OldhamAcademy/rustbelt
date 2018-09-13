
fn main() {
    let matches = App::new("AntiRustMac")
        .version("0.1.0")
        .author("Kyle Oldham <kyleoldham.4@gmail.com>")
        .about("Rust Mac Setup")
        .arg(Arg::with_name("")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("eh"))
        .get_matches();
    let url = matches.value_of("URL").unwrap();
    println!("{}", url);
}
