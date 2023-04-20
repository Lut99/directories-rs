//  SHOWCASE.rs
//    by Lut99
// 
//  Created:
//    20 Apr 2023, 19:08:02
//  Last edited:
//    20 Apr 2023, 19:22:24
//  Auto updated?
//    Yes
// 
//  Description:
//!   Shows a few based aspects from the library.
// 

use std::path::PathBuf;

use clap::Parser;
use humanlog::{DebugMode, HumanLogger};

use directories::Directory;


/***** DIRECTORIES *****/
/// Defines the directory structure we use for testing.
#[derive(Debug, Directory)]
struct RootDir {
    /// A hardcoded file in this directory
    #[file(path = "Test.txt")]
    test        : PathBuf,
    /// A nested folder with a random name
    #[dir(path = "HelloWorld")]
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
    #[file(optional)]
    optional_file_dat : PathBuf,
}

/// Defines the layout of the TestCasesDir, which can hold a variable number of directories.
#[derive(Debug, Directory)]
struct TestCasesDir {
    /// A hardcoded file which we always expect
    #[file(optional, path = "hardcoded.exe")]
    hardcoded_exe : PathBuf,
    /// A variable directory, which only matches things of this shape
    test_cases    : Vec<TestCaseDir>,
    /// A variable list of files, which only matches files
    #[file(any)]
    test_files    : Vec<PathBuf>,
    /// A variable list that matches anything else
    #[path(any)]
    rest          : Vec<PathBuf>,
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

    // 
}
