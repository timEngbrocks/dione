#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(strict_provenance)]

use getopts::Occur;
use args::{ArgsError, Args};

use jvm::JVM;

pub mod class_loader;
pub mod jvm;
pub mod util;
pub mod macros;

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
		jvm!().initialize(jdk_base_path);
        jvm!().run(vec![class]);
	}

	let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

const PROGRAM_DESC: &'static str = "A JVM written in Rust";
const PROGRAM_NAME: &'static str = "dione";

fn parse(input: &Vec<String>) -> Result<Option<(String, String)>, ArgsError> {

    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);

    args.flag("h", "help", "Print the usage menu");

    args.option("c",
        "class",
        ".class file to run",
        "Class",
        Occur::Req,
        None);

    args.option("j",
        "jdk",
        "Path to the JDK",
        "JDK",
        Occur::Req,
        None);

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        args.full_usage();
        return Ok(None);
    }

    let class: String = args.value_of::<String>("class").expect("Missing class argument");
    let jdk: String = args.value_of::<String>("jdk").expect("Missing JDK argument");

    Ok(Some((class, jdk)))
}
