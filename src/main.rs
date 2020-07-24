mod lines;

use structopt::StructOpt;
use indicatif::ProgressBar;
use console::style;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "You can pass in patterns as just a word, \
    or use a phrase by surrounding your argument with single quotes 'like this'!")]
    pattern: String,
    #[structopt(parse(from_os_str), help = "Here you can pass the path to *almost* \
     any file that's readable as text.")]
    path: std::path::PathBuf,
}

pub struct ResultSet {
    result_matches: Vec<Vec<String>>
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    use crate::lines::parse_lines;

    let from_args = Cli::from_args();

    let result = std::fs::read_to_string(&from_args.path);

    let _content = match result {
        Ok(content) => { content }
        Err(error) => { return Err(error.into()); }
    };

    let pb = &ProgressBar::new(_content.lines().count() as u64);

    let result_set = &mut ResultSet { result_matches: vec![] };

    let pattern = format!("{}", &from_args.pattern);

    let line_count = _content.lines().enumerate().count();

    parse_lines(pattern, pb, result_set, _content);

    pb.finish_and_clear();

    let result_or_results =
        if *&result_set.result_matches.len() as u32 == 1 { "result" } else { "results" };


    println!("Found {} matching {} in {} total lines...",
             style(&result_set.result_matches.len()).bold().green(),
             result_or_results,
             format!("{}", line_count));
    println!();

    for result in &result_set.result_matches {

            println!("{}: {}", style(result[1].to_string()).cyan(), result[0])

    }

    Ok(())

}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {

    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        panic!("Ahh! Watch those wrist rockets!")
    }

}
