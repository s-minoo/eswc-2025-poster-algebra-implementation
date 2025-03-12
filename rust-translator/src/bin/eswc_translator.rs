use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use translator::io;
use translator::translator::translate_normalized_rml;

use clap::{arg, Command};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cli {
    pub cmd: Command,
}

impl Default for Cli {
    fn default() -> Self {
        let cmd = Command::new("ESWCO 2025 Translator")
            .version(VERSION)
            .author("Sitt Min Oo")
            .about(format!("Translates mapping documents to execution plans consisting of algebraic mapping operators.\n\
                Current version {} supports RML translation as described in the ESWC 2025 paper", VERSION))
            .subcommand_required(true)
            .propagate_version(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("file")
                         .about("translate a single mapping document")
                         .arg(arg!(<DOCUMENT> "the mapping document to be translated"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("folder")
                         .about("translate all mapping documents under the given folder")
                         .arg(arg!(<FOLDER> "the folder containing several mapping documents"))
                         .arg_required_else_help(true))
            .subcommand(Command::new("stdin")
                         .about("translate all input from stdin"))
            .arg(arg!(-d --debug ...  "Turns on debugging and logging to file"))
            .arg(arg!(-j --json ... "Only generate dot files in JSON format"))
            .arg(arg!(-o --outputFolderSuffix <OUTPUT_FOLDER_SUFFIX> "The output folder suffix"));

        Self { cmd }
    }
}
pub fn main() -> anyhow::Result<()> {
    let cli = Cli::default();
    let matches = cli.cmd.get_matches();
    let mut output_folder = PathBuf::from_str("./")?;
    if let Some(arg_out_folder) = matches.get_one::<String>("outputFolderSuffix") {
        output_folder = Path::new(arg_out_folder).to_path_buf();
    }

    if let Some((cmd, sub_args)) = matches.subcommand() {
        match cmd {
            "file" => {
                let file_name: &Path = sub_args
                    .get_one::<String>("DOCUMENT")
                    .map(Path::new)
                    .expect("expects a document file path");

                handle_file(output_folder, file_name)?;
            }
            "folder" => {
                let root = sub_args
                    .get_one::<String>("FOLDER")
                    .map(Path::new)
                    .expect("expects a folder path");
                let files = walkdir::WalkDir::new(root)
                    .into_iter()
                    .filter_map(|entry_res| entry_res.ok())
                    .map(|entry| entry.into_path())
                    .filter(|path| path.is_file());

                for file_path in files {
                    if let Some(file_name) = file_path.file_name() {
                        if file_name.to_string_lossy().contains("normalized.ttl") {
                            let parent_folder = file_path.parent().unwrap_or(Path::new("./"));
                            println!("Processing RML document: {}", file_path.to_string_lossy());
                            handle_file(parent_folder.to_path_buf(), &file_path)?;
                            println!("{}", "=".repeat(10));
                        }
                    }
                }
            }
            "stdin" => {}
            _ => {}
        }
    }

    Ok(())
}

fn handle_file(output_folder: PathBuf, file_name: &Path) -> Result<(), anyhow::Error> {
    let output_plan_json = output_folder.join("plan.json");
    let output_plan_dot = output_folder.join("plan.dot");
    let output_plan_pretty_dot = output_folder.join("plan_pretty.dot");

    let store = io::read_rml_document(file_name)?;
    //let normalized_store = normalize_rml(&store)?;

    let base_iri = get_base_iri(file_name);
    println!("Base IRI is {:#?}", base_iri);
    let mut plan = translate_normalized_rml(&store, base_iri)?;

    println!("Writing plan.dot file....");
    plan.write(output_plan_dot).unwrap();

    println!("Writing plan_pretty.dot file....");
    plan.write_pretty(output_plan_pretty_dot).unwrap();

    println!("Writing plan.json file....");
    plan.write_json(output_plan_json).unwrap();

    Ok(())
}

fn get_base_iri(file_name: &Path) -> Option<String> {
    let file = File::open(file_name).unwrap();
    let regex = Regex::new(r"<(.*)>").unwrap();
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|line| line.to_lowercase().contains("@base"))
        .filter_map(|line| regex.captures(&line).map(|mat| mat[1].to_string()))
        .next()
}
