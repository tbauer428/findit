use structopt::StructOpt;
use indicatif::ProgressBar;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "You can pass in patterns as just a word, or use a phrase by surrounding your argument with single quotes 'like this'!")]
    pattern: String,
    #[structopt(parse(from_os_str), help = "Here you can pass the path to *almost* any file that's readable as text.")]
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

    let result_or_results = if *&result_set.result_matches.len() as u32 == 1 { "result" } else { "results" };

    println!("Found {} matching {} in {} total lines...",
             &result_set.result_matches.len(),
             result_or_results,
             format!("{}", line_count));
    for result in &result_set.result_matches {

            println!("{}: {}", result[1], result[0])

    }

    Ok(())

}
