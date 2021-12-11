use std::fs;

#[derive(Debug, PartialEq)]
enum LineEvaluation {
    Ok,
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn main() {
    // Read the input file
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let syntax_score = syntax_error_score(input.clone());

    println!("Syntax error score: {}", syntax_score);

    let middle_score = auto_complete_score(input);

    println!("Auto complete score: {}", middle_score);
}

fn parse_line(line: String) -> LineEvaluation {
    let mut stack = vec![];

    for ch in line.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    if ch != last {
                        return LineEvaluation::Corrupt(ch);
                    }
                } else {
                    return LineEvaluation::Corrupt(ch);
                }
            }
            _ => return LineEvaluation::Corrupt(ch),
        }
    }

    if stack.len() > 0 {
        LineEvaluation::Incomplete(stack.into_iter().rev().collect())
    } else {
        LineEvaluation::Ok
    }
}

fn syntax_error_score(input: String) -> usize {
    let mut score = 0;

    for line in input.trim().split('\n') {
        if let LineEvaluation::Corrupt(ch) = parse_line(line.to_string()) {
            score += match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            };
        }
    }

    score
}

fn auto_complete_score(input: String) -> usize {
    let mut scores: Vec<usize> = input
        .trim()
        .split('\n')
        .map(|line| parse_line(line.to_string()))
        .filter(|eval| match eval {
            LineEvaluation::Incomplete(_) => true,
            _ => false,
        })
        .map(|eval| {
            if let LineEvaluation::Incomplete(chars) = eval {
                let mut score = 0;

                for ch in chars {
                    score *= 5;
                    score += match ch {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    };
                }

                score
            } else {
                panic!("Unreachable!");
            }
        })
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::{auto_complete_score, parse_line, syntax_error_score, LineEvaluation};

    #[test]
    fn should_parse_corrupt_line() {
        let input = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
        let expected = LineEvaluation::Corrupt('}');
        let actual = parse_line(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_ok_line() {
        let input = "[<>({}){}[([])<>]]".to_string();
        let expected = LineEvaluation::Ok;
        let actual = parse_line(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_incomplete_line() {
        let input = "[({(<(())[]>[[{[]{<()<>>".to_string();
        let expected = LineEvaluation::Incomplete(vec!['}', '}', ']', ']', ')', '}', ')', ']']);
        let actual = parse_line(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_syntax_error_score() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .to_string();

        let actual = syntax_error_score(input);

        assert_eq!(actual, 26397);
    }

    #[test]
    fn should_determine_auto_complete_score() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .to_string();

        let actual = auto_complete_score(input);

        assert_eq!(actual, 288957);
    }
}
