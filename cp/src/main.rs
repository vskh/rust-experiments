use log::{debug, error, info, trace, warn};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(name = "Copy", version = "0.1", author)]
/// File copy utility.
///
/// Subset of what UNIX cp can do written in Rust.
struct Opts {
    #[structopt(
        short,
        long,
        max_values = 3,
        takes_value = false,
        parse(from_occurrences)
    )]
    /// Enable debug logging.
    pub verbose: u8,

    #[structopt(min_values = 2)]
    /// Paths to process.
    ///
    /// There can be one or more source paths and one (last) destination path.
    pub paths: Vec<String>,
}

fn init_log(debug_level: u8) -> Result<()> {
    let encoder = PatternEncoder::new("{h({l} {t}: {m}{n})}");
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .build(
            Root::builder()
                .appender("console")
                .build(match debug_level {
                    0 => log::LevelFilter::Warn,
                    1 => log::LevelFilter::Info,
                    2 => log::LevelFilter::Debug,
                    _ => log::LevelFilter::Trace,
                }),
        )
        .map_err(|e| format!("Logger configuration error: {}", e))?;
    log4rs::init_config(config).map_err(|e| format!("Logger initialization error: {}", e))?;
    Ok(())
}

fn copy_file(src: &Path, dst: &Path) -> Result<u64> {
    let mut src_file = File::open(src)?;
    let mut dst_file = File::create(dst)?;

    let bytes_copied = io::copy(&mut src_file, &mut dst_file)?;

    Ok(bytes_copied)
}

fn run(mut opts: Opts) -> Result<()> {
    init_log(opts.verbose)?;

    let dst = opts.paths.pop().unwrap();
    let srcs = opts.paths;

    trace!("Sources: {:?}", srcs);
    trace!("Destination: {:?}", dst);

    let mut dest_is_dir = false;
    if srcs.len() > 1 {
        match fs::metadata(&dst) {
            Ok(dst_meta) if dst_meta.is_dir() => Ok(()),
            _ => Err(format!(
                "specified destination '{}' is not a directory",
                dst
            )),
        }?;

        dest_is_dir = true;
    }

    let mut total_bytes_copied = 0;
    for src in srcs {
        debug!("'{}' -> '{}'", src, dst);

        let src_path = PathBuf::from(src);
        let mut dst_path = PathBuf::from(&dst);

        if dest_is_dir {
            dst_path.push(src_path.file_name().unwrap())
        }

        let bytes_copied = copy_file(src_path.as_path(), dst_path.as_path())?;
        trace!("Copied {} bytes", bytes_copied);

        total_bytes_copied += bytes_copied
    }

    info!("Copied {} bytes", total_bytes_copied);

    Ok(())
}

fn main() {
    let opts = Opts::from_args();

    debug!("{:?}", opts);

    match run(opts) {
        Err(e) => error!("{}", e),
        _ => (),
    }
}
