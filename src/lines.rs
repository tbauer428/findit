use crate::ResultSet;
use indicatif::ProgressBar;

pub fn parse_lines(
    pattern: String,
    pb: &ProgressBar,
    result_set: &mut ResultSet,
    _content: String,
) {
    let mut line_count = 0;

    for line in _content.lines() {
        if line.contains(&pattern) {
            line_count = line_count + 1;

            let mut new_vec = vec![];

            new_vec.push(line.parse().unwrap());

            new_vec.push(format!("{}", line_count));

            result_set.result_matches.push(new_vec);

            pb.inc(1);
        } else {
            line_count = line_count + 1;

            pb.inc(1);
        }
    }
}
