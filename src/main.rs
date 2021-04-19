use lcov_diff::diff_reports;

use std::error::Error;
use std::path::{Path, PathBuf};

use lcov::Report;
use log::{debug, info};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;
use structopt::StructOpt;
use tempfile::NamedTempFile;

/// Defines CLI structure
#[derive(Debug, StructOpt)]
#[structopt(
    name = "lcov-diff",
    about = "lcov-diff utility to get diff of two lcov files"
)]
struct Cli {
    #[structopt(short, long, help = "Show Debug logging")]
    verbose: bool,

    #[structopt(
        name = "PATH",
        short = "o",
        long = "output",
        help = "output file to write (stdout if not presented)",
        parse(from_os_str)
    )]
    output: Option<PathBuf>,

    #[structopt(
        short = "w",
        long = "web",
        help = "Generate html report from output file (default name web)"
    )]
    web: Option<Option<PathBuf>>,

    /// Files to process, right now just two of them
    #[structopt(
        name = "FILE",
        parse(from_os_str),
        required = true,
        min_values = 2,
        max_values = 2
    )]
    files: Vec<PathBuf>,
}

fn genhtml<P: AsRef<Path>>(lcov_path: P, report_dir: &str) -> bool {
    info!("Generate html report for the {:?}", lcov_path.as_ref());
    let mut out = Command::new("genhtml")
        .args(&[
            "--ignore-errors",
            "source",
            "-o",
            report_dir,
            lcov_path.as_ref().to_str().unwrap(),
        ])
        .spawn()
        .expect("failed to execute html report generation");
    let status = out
        .wait()
        .expect("Something went wrong during waiting genhtml...");
    info!("Success: {}", status.success());
    status.success()
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Cli::from_args();

    env_logger::builder()
        .filter_level(match options.verbose {
            true => log::LevelFilter::Debug,
            false => log::LevelFilter::Info,
        })
        .init();

    debug!("{:#?}", options);

    info!("Processing diff for two lcov files: {:?}", options.files);

    let report = diff_reports(
        &Report::from_file(options.files[0].as_path())?,
        &Report::from_file(options.files[1].as_path())?,
    )?;

    match &options.output {
        Some(output) => {
            let out_file = output.to_owned();
            info!("Writing diff to file: {:?} ...", out_file);
            let file = File::create(output).unwrap();
            let mut writer = BufWriter::new(file);
            report
                .into_records()
                .for_each(|rec| writeln!(writer, "{}", rec).unwrap());
            writer.flush()?;

            options.web.map(|rep_dir| {
                let report_path = match &rep_dir {
                    Some(dir) => dir.to_str().unwrap(),
                    None => "web",
                };
                genhtml(&output, report_path);
                Some(())
            });
        }
        None => match &options.web {
            Some(rep_dir) => {
                let file = NamedTempFile::new()?;
                let mut writer = BufWriter::new(file.as_file());
                report
                    .into_records()
                    .for_each(|rec| writeln!(writer, "{}", rec).unwrap());
                writer.flush()?;

                let report_path = match &rep_dir {
                    Some(dir) => dir.to_str().unwrap(),
                    None => "web",
                };
                genhtml(file.path(), report_path);
            }
            None => {
                info!("Writing diff to stdout");
                report.into_records().for_each(|rec| println!("{}", rec));
            }
        },
    };

    Ok(())
}
