//  SHOWCASE.rs
//    by Lut99
// 
//  Created:
//    20 Apr 2023, 19:08:02
//  Last edited:
//    24 Jun 2023, 14:03:35
//  Auto updated?
//    Yes
// 
//  Description:
//!   Shows a few based aspects from the library.
// 

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use humanlog::{DebugMode, HumanLogger};
use log::error;

use directories::{Directory, DirectoryExt as _};
use directories::std::Dynamic;


/***** DIRECTORIES *****/
/// Defines the directory structure we use for testing.
#[derive(Debug, Directory)]
struct RootDir {
    /// A hardcoded file in this directory
    #[file(path = "Test.txt")]
    test        : PathBuf,
    /// A nested folder with a random name
    #[dir(path = "HelloWorld", path = "HelloWorld2")]
    hello_world : HelloWorldDir,
}

/// Defines the layout of the nested "HelloWorld" directory
#[derive(Debug, Directory)]
struct HelloWorldDir {
    /// A hardcoded, nested file
    nested_test_txt   : PathBuf,
    /// A directory containing variable stuff
    test_cases        : TestCasesDir,
    /// An optional, extra file!
    optional_file_dat : Option<PathBuf>,
}

/// Defines the layout of the TestCasesDir, which can hold a variable number of directories.
#[derive(Debug, Directory)]
struct TestCasesDir {
    /// A hardcoded file which we always expect
    #[file(path = "hardcoded.exe")]
    hardcoded_exe : PathBuf,
    /// A variable directory, which matches any nested file/folder
    #[dir(path = "/home")]
    test_cases        : HashMap<PathBuf, TestCaseDir>,
    /// A variable directory, which matches anything only of the given shape
    #[dir(path = "/home")]
    test_cases_strict : Dynamic<TestCaseDir>,
//     /// A variable list of files, which only matches files
//     #[file(any)]
//     test_files    : Vec<PathBuf>,
//     /// A variable list that matches anything else
//     #[dir(any)]
//     rest          : Vec<PathBuf>,
}

/// Defines the layout of one of these hypothetical test cases.
#[derive(Debug, Directory)]
struct TestCaseDir {
    lut_99 : PathBuf,
}





/***** ARGUMENTS *****/
/// Defines arguments for this testcase
#[derive(Parser)]
struct Arguments {
    /// Whether to enable full debug structures or not
    #[clap(long, global=true, help="If given, enables additional logging statements.")]
    debug : bool,
}





/***** ENTYRPOINT *****/
fn main() {
    // Parse the CLI
    let args: Arguments = Arguments::parse();

    // Initialize the logger
    if let Err(err) = HumanLogger::terminal(if args.debug { DebugMode::Debug } else { DebugMode::HumanFriendly }).init() {
        eprintln!("WARNING: Failed to initialize logger: {err} (no logging enabled for this session)");
    }

    // Initialize the paths
    let root_dir: RootDir = match RootDir::try_init("./") {
        Ok(dir)  => dir,
        Err(err) => { error!("Failed to initialize root directory: {err}"); std::process::exit(1); },
    };

    // Show the entire thing
    println!("Directory structure:");
    println!("{}", root_dir.display());

    // Check what we found
    println!("Path to test file: {}", root_dir.test.display());
    println!("Path to nested test file: {}", root_dir.hello_world.nested_test_txt.display());
    println!("Path to optional file: {:?}", root_dir.hello_world.optional_file_dat.as_ref().map(|p| p.display().to_string()));
    println!("Path to testcases:");
    for (path, _) in &root_dir.hello_world.test_cases.test_cases {
        println!(" - {}", path.display());
    }
    println!("Path to testcases (strict):");
    for (path, _) in &root_dir.hello_world.test_cases.test_cases_strict {
        println!(" - {}", path.display());
    }
}
