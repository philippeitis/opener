#![warn(
    rust_2018_idioms,
    deprecated_in_future,
    macro_use_extern_crate,
    missing_debug_implementations,
    unused_qualifications
)]

use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    /// The path to open
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Open the path with the `open_browser()` function
    #[structopt(long = "browser")]
    browser: bool,

    /// Open the path with the `open_in_file_manager()` function
    #[structopt(long = "file-manager", conflicts_with("browser"))]
    file_manager: bool,
}

fn main() {
    let args = Args::from_args();

    let open_result = if args.browser {
        opener::open_browser(&args.path)
    } else if args.file_manager {
        opener::open_in_file_manager(&args.path)
    } else {
        opener::open(&args.path)
    };

    match open_result {
        Ok(()) => {
            println!("Opened path successfully.");
        }
        Err(e) => {
            println!("Failed to open path.\n\nerror:\n\n{:#?}", e);
            process::exit(1);
        }
    }
}
