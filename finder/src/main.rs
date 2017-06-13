extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("finder")
                        .version("0.0.1")
                        .author("Pradeep Chhetri")
                        .about("Finds if any version control leak is present")
                        .arg(Arg::with_name("input")
                                    .short("i")
                                    .long("input")
                                    .value_name("INPUT_FILE")
                                    .help("Input file containing domain names")
                                    .takes_value(true))
                        .arg(Arg::with_name("output")
                                    .short("o")
                                    .long("output")
                                    .value_name("OUTPUT_FILE")
                                    .help("Output file containing infected domain names")
                                    .takes_value(true))
                        .get_matches();

    if let Some(i) = matches.value_of("input") {
        println!("Value of input file: {}", i);
    }

    if let Some(o) = matches.value_of("output") {
        println!("Value of output file: {}", o);
    }
}