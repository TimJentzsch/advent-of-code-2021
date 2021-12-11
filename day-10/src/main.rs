#[derive(Debug, PartialEq)]
enum LineEvaluation {
    Ok,
    Corrupt(char),
    Incomplete,
}

fn main() {
    println!("Hello, world!");
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
        LineEvaluation::Incomplete
    } else {
        LineEvaluation::Ok
    }
}

#[cfg(test)]
mod tests {
    use crate::{LineEvaluation, parse_line};

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
        let input = "[<>({}){}[([])<>]".to_string();
        let expected = LineEvaluation::Incomplete;
        let actual = parse_line(input);

        assert_eq!(actual, expected);
    }
}
