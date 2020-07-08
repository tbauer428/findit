use structopt::StructOpt;
use std::collections::LinkedList;
use std::fmt;
use indicatif::ProgressBar;
use std::fmt::{Formatter, Debug};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct ResultSet {
    result_matches: Vec<Vec<String>>
}


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);

    let _content = match result {
        Ok(content) => { content }
        Err(error) => { return Err(error.into()); }
    };

    let pb = ProgressBar::new(_content.lines().count() as u64);

    let mut result_set = ResultSet { result_matches: vec![] };

    for line in _content.lines() {
        if line.contains(&args.pattern) {

            let mut new_vec = vec![];

            new_vec.push(line.parse().unwrap());

            let line_number = _content.find(line.parse().unwrap()).unwrap() as u32;
            new_vec.push(line_number.to_string());


            result_set.result_matches.push(new_vec);

            pb.inc(1);
        }
        pb.inc(1);
    }

    pb.finish();

    println!("Found {} matching results", &result_set.result_matches.len());
    for result in &result_set.result_matches {

            println!("{}, {}", result[0], result[1])


    }

    Ok(())

}
