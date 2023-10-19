#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(strict_provenance)]

use std::path::{PathBuf, Path};

use getopts::Occur;

use args::{ArgsError, Args};
use class_loader::ClassLoader;

use crate::util::file_util::list_files;

mod class_loader;
mod jvm;
mod util;

fn main() {
    let (class, jdk) = match parse(&std::env::args().collect()) {
        Ok(Some((class, jdk))) => (class, jdk),
        Ok(None) => return,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

	let jdk_file_paths = list_files(Path::new(&jdk));
	let jdk: Vec<&str> = jdk_file_paths.iter().map(|pathbuf: &PathBuf| {
		match pathbuf.to_str() {
			Some(v) => v,
			None => panic!("Failed to load JDK"),
		}
	}).collect();

	use std::time::Instant;
    let now = Instant::now();
	{
		ClassLoader::load(vec![&class], vec![jdk]);
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
