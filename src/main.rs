use structopt::StructOpt;
use indicatif::ProgressBar;

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

    let mut line_count = 0;

    for line in _content.lines() {
        if line.contains(&args.pattern) {

            line_count = line_count + 1;

            let mut new_vec = vec![];

            new_vec.push(line.parse().unwrap());

            new_vec.push(format!("{}", line_count));

            result_set.result_matches.push(new_vec);

            pb.inc(1);

        }else {
            line_count = line_count + 1;

            pb.inc(1);

        }
    }

    pb.finish();

    println!("Found {} matching results in {} total lines...",
             &result_set.result_matches.len(),
             format!("{}", line_count));
    for result in &result_set.result_matches {

            println!("{}: {}", result[1], result[0])

    }

    Ok(())

}
