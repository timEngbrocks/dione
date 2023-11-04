#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(strict_provenance)]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::len_zero,
    clippy::cast_enum_constructor,
    clippy::fn_to_numeric_cast_with_truncation
)]

use args::{Args, ArgsError};
use getopts::Occur;

use jvm::JVM;

pub mod class_loader;
pub mod jvm;
pub mod macros;
pub mod util;

fn main() {
    let (class, jdk_base_path) = match parse(&std::env::args().collect()) {
        Ok(Some((class, jdk))) => (class, jdk),
        Ok(None) => return,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    use std::time::Instant;
    let now = Instant::now();

    {
        JVM::start(jdk_base_path);
        JVM::run(vec![class]);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

const PROGRAM_DESC: &str = "A JVM written in Rust";
const PROGRAM_NAME: &str = "dione";

fn parse(input: &Vec<String>) -> Result<Option<(String, String)>, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);

    args.flag("h", "help", "Print the usage menu");

    args.option(
        "c",
        "class",
        ".class file to run",
        "Class",
        Occur::Req,
        None,
    );

    args.option("j", "jdk", "Path to the JDK", "JDK", Occur::Req, None);

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        args.full_usage();
        return Ok(None);
    }

    let class: String = args
        .value_of::<String>("class")
        .expect("Missing class argument");
    let jdk: String = args
        .value_of::<String>("jdk")
        .expect("Missing JDK argument");

    Ok(Some((class, jdk)))
}
