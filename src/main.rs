use clap::{command, value_parser, Arg, ArgAction, ArgMatches};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::path::PathBuf;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let matches: ArgMatches = command!()
        .author("Maurice-Jörg Nießen <info@mjniessen.com>")
        .arg(Arg::new("FILE").value_parser(value_parser!(PathBuf)))
        .arg(
            Arg::new("WIDTH")
                .short('w')
                .long("width")
                .default_value("80")
                .value_parser(value_parser!(u16).range(10..))
                .help("Wrap within <WIDTH> characters"),
        )
        .arg(
            Arg::new("TERMINAL")
                .short('t')
                .long("terminal")
                .action(ArgAction::SetTrue)
                .conflicts_with("WIDTH")
                .help("Set <WIDTH> to size of actual terminal"),
        )
        .arg(
            Arg::new("EOL")
                .short('e')
                .long("eol")
                .value_parser(value_parser!(String))
                .help("end of line character"),
        )
        .arg(
            Arg::new("OUTPUT")
                .short('o')
                .long("output")
                .value_parser(value_parser!(PathBuf))
                .conflicts_with("TERMINAL")
                .help("Write to <OUTPUT> instead of STDOUT"),
        )
        .get_matches();

    // define width
    let mut width: u16 = 80;

    if matches.get_flag("TERMINAL") {
        let size = terminal_size();
        if let Some((Width(terminal_width), Height(_))) = size {
            width = terminal_width;
        } else {
            // TODO: warning or error output to STDERR
            println!("Unable to get terminal size");
            // error/warning, but width is set to 80
        }
    } else if matches.contains_id("WIDTH") {
        let given_width = matches.get_one::<u16>("WIDTH").unwrap();
        width = *given_width;
    }

    // define eol-character/string
    let mut eol: &str = "";

    if matches.contains_id("EOL") {
        let given_eol = matches.get_one::<String>("EOL").unwrap();
        eol = given_eol;
    }

    // TODO: multiple INPUTs / files
    // define input
    if matches.contains_id("FILE") {
        let input = matches.get_one::<PathBuf>("FILE");

        let f = File::open(input.unwrap()).unwrap();
        // TODO: error handling - exists file and is readable
        let mut r = BufReader::new(f);
        process_reader(&mut r, width as usize, eol);
    } else if atty::isnt(atty::Stream::Stdin) {
        let f = stdin();
        let mut r = BufReader::new(f);
        process_reader(&mut r, width as usize, eol);
    }

    // println!("stdout? {}", is(Stream::Stdout));
    // println!("stderr? {}", is(Stream::Stderr));
    // println!("stdin? {}", is(Stream::Stdin));

    // for input_file in args.input_files {
    //     if input_file.as_os_str() == "-" {
    //         let f = stdin();
    //         let mut r = BufReader::new(f);
    //         process_reader(&mut r, args.width, &args.eol);
    //     } else {
    //         let f = File::open(&input_file).unwrap();
    //         let mut r = BufReader::new(f);
    //         process_reader(&mut r, args.width, &args.eol);
    //     }
    // }
}

// fn read_pipe() -> Option<String> {
//     let mut buf = String::new();
//     if atty::isnt(atty::Stream::Stdin) {
//         std::io::stdin().read_to_string(&mut buf).ok()?;
//     }
//     (!buf.is_empty()).then_some(buf.trim().into())
// }

fn chomp(s: &str) -> String {
    let mut s = s.to_string();
    while s.ends_with('\n') {
        s.pop();
        while s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

fn process_line(line: &str, width: usize, eol: &str) {
    let line = chomp(line);
    let lines = textwrap::wrap(&line, width);
    let last = lines.len() - 1;
    for (i, line) in lines.iter().enumerate() {
        if i == last {
            println!("{line}");
        } else {
            println!("{line}{eol}");
        }
    }
}

fn process_reader<R>(r: &mut BufReader<R>, width: usize, eol: &str)
where
    R: Read,
{
    let mut line = String::new();
    while let Ok(n) = r.read_line(&mut line) {
        if n == 0 {
            break;
        } else {
            process_line(&line, width, eol);
            line = String::new();
        }
    }
}
