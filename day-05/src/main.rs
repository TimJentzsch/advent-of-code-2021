#[derive(Debug, PartialEq)]
struct ParseError(String);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    /// Parse a point from an input string.
    /// 
    /// Example string:
    /// 
    /// `3,4`
    fn from_input(input: String) -> Result<Point, ParseError> {
        let mut parts = input.split(",").into_iter();

        // Parse x
        let x: u32 = if let Some(x_part) = parts.next() {
            if let Ok(x) = x_part.parse() {
                x
            } else {
                return Err(ParseError(input));
            }
        } else {
            return Err(ParseError(input));
        };

        // Parse y
        let y: u32 = if let Some(y_part) = parts.next() {
            if let Ok(x) = y_part.parse() {
                x
            } else {
                return Err(ParseError(input));
            }
        } else {
            return Err(ParseError(input));
        };

        Ok(Point { x, y })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::Point;

    #[test]
    fn should_parse_point_from_input() {
        let input = "3,4".to_string();
        let expected = Ok(Point::new(3, 4));
        let actual = Point::from_input(input);

        assert_eq!(actual, expected);
    }
}
