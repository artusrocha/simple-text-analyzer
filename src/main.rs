use sta::analysis::{Analysis, AnalysisSummary};
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let summary = match args.get(1) {
        Some(filename) => process_file(filename),
        None => process_stdin(),
    };

    eprintln!("=============================");
    eprintln!("");

    println!("{}", serde_json::to_string(&summary).unwrap());
}

fn process_file(filename: &str) -> AnalysisSummary {
    eprintln!("Reading from file: {}", &filename);
    let contents = fs::read_to_string(filename).expect("Error reading file");
    let mut analysis = Analysis::new();
    analysis.process_sample(&contents);
    analysis.get_summary()
}

fn process_stdin() -> AnalysisSummary {
    eprintln!("Reading from STDIN");
    let mut analysis = Analysis::new();
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n_bytes_readed) => {
                if n_bytes_readed == 0 {
                    eprintln!("End of Input");
                    break;
                }
                analysis.process_sample(&input);
            }
            Err(error) => eprintln!("Error: {}", error),
        }
        input.clear();
    }
    analysis.get_summary()
}
