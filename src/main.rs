mod lines;

use std::time::SystemTime;

use console::style;
use indicatif::ProgressBar;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "You can pass in patterns as just a word, \
    or use a phrase by surrounding your argument with single quotes 'like this'!")]
    pattern: String,
    #[structopt(
        parse(from_os_str),
        help = "Here you can pass the path to *almost* \
     any file that's readable as text."
    )]
    path: std::path::PathBuf,
    #[structopt(
        short = "s",
        long = "save",
        help = "Save the results to a file, default is ./saved/<UNIXTIME>.txt"
    )]
    save: bool,
}

pub struct ResultSet {
    result_matches: Vec<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::lines::parse_lines;

    let from_args = Cli::from_args();

    let result = std::fs::read_to_string(&from_args.path);

    let _content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    let pb = &ProgressBar::new(_content.lines().count() as u64);

    let result_set = &mut ResultSet {
        result_matches: vec![],
    };

    let pattern = format!("{}", &from_args.pattern);

    let line_count = _content.lines().enumerate().count();

    parse_lines(pattern, pb, result_set, _content);

    pb.finish_and_clear();

    let result_or_results = if *&result_set.result_matches.len() as u32 == 1 {
        "result"
    } else {
        "results"
    };

    println!(
        "Found {} matching {} in {} total lines... \n",
        style(&result_set.result_matches.len()).bold().green(),
        result_or_results,
        format!("{}", line_count)
    );

    let mut content = "".to_string();

    for result in &result_set.result_matches {
        let formatted = format!("{}: {} \n", result[1].to_string(), result[0]);
        content.push_str(&formatted);
        println!("{}: {}", style(result[1].to_string()).cyan(), result[0])
    }

    if from_args.save {
        save(content)
    }

    Ok(())
}

fn save(content: String) {
    if !std::path::Path::new("saved").exists() {
        match std::fs::create_dir("saved") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {}
        }
    }

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    std::fs::write(format!("./saved/{:?}.txt", in_ms), content).unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {}

    #[test]
    #[should_panic]
    fn test_any_panic() {
        panic!("Ahh! Watch those wrist rockets!")
    }
}
