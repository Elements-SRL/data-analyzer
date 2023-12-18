use std::{path::PathBuf, fs, io};
use clap::Parser;
use channel_selector::ChannelSelector;
use my_abf::MyAbf;
use csv::Writer;
use rust_abf::AbfBuilder;

mod my_abf;
mod channel_selector;

#[derive(Parser)]
struct Cli {
    abf_path: std::path::PathBuf,
    csv_path: std::path::PathBuf,
}

impl  Cli {
    fn new(abf_path: std::path::PathBuf, csv_path: std::path::PathBuf) -> Self {
        Self { abf_path, csv_path }
    }
}

fn find_abfs(dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .flat_map(|path| {
                    if path.is_dir() {
                        find_abfs(&path)
                    } else {
                        vec![path]
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}


fn main() {
    let args = Cli::parse();
    let mut csv_path = args.csv_path;
    // Append a filename to the PathBuf
    let filename = "results.csv";
    csv_path.push(filename);
    let csv_path = csv_path;
    // Prompt the user for input
    print!("Enter the channels you want to analyze:
all - all the channels
index_of_channel - only one channel
");

    // Create a String to store the user's input
    let mut input = String::new();

    let cs =  match io::stdin().read_line(&mut input) {
        Ok(_) => ChannelSelector::from_str(&input),
        Err(error) => return eprintln!("Error reading input: {}", error)
    };

    let Ok(cs) = cs else {
        return eprintln!("Something wrogn with the input");
    };

    let mut wtr = Writer::from_writer(vec![]);
    find_abfs(&args.abf_path)
    .iter()
    .map(|p| {
        let Some(f_name) = p.file_name() else {
            return None;
        };
        let Some(p_str) = p.to_str() else {
            return None;
        };
        Some((f_name, p_str))
    })
    .flatten()
    .flat_map(|p| {
        let Ok(abf) = AbfBuilder::from_file(p.1) else {
            return None;
        };
        let Some(fname) = p.0.to_str() else {
            return None;
        };
        Some(MyAbf::new (abf, fname))
    })
    .map(|my_abf| my_abf.get_analysis_result(&cs) )
    .flatten()
    .for_each(|r| wtr.serialize(r).unwrap());
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    fs::write(csv_path, data).expect("Unable to write file");
}