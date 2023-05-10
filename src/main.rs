use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(ValueEnum, Clone, Debug)]
enum Type {
    RELEASE,
    DEBUG,
    FAKE,
}

/// This is a program to generate a manifest
#[derive(Parser, Debug)]
#[command(author, version = VERSION, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    #[clap(required = true)]
    files: Vec<std::path::PathBuf>,

    /// Number of times to greet
    #[arg(short, long)]
    #[clap(required = true)]
    output: String,

    /// Type
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=Type::DEBUG)]
    type_: Type,
}

fn main() {
    let args: Args = Args::parse();
    println!("Welcome to the MANIFEST generator vesion {}\n\nArguments:\n", VERSION);
    println!("-> Output {}\n-> Type: {:#?}", args.output, args.type_);
    if let Type::FAKE = args.type_ {
        panic!("The FAKE types force to a panic!!!!");
    }
    let mut out_file: File = File::create(args.output).unwrap();
    writeln!(&mut out_file, "MANIFEST v{}\n\nType: {:?}\n\nFiles:", VERSION, args.type_).unwrap();
    for f in args.files {
        println!("*-> File: {:?}", f);
        writeln!(&mut out_file, "- {:?}", f).unwrap();
    }
    drop(out_file);
}