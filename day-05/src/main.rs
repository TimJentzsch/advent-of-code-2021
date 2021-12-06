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

    /// Format the point to a string.
    fn stringify(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    /// Parse a line from an input string.
    ///
    /// Example string:
    ///
    /// `3,4 -> 1,4`
    fn from_input(input: String) -> Result<Line, ParseError> {
        let mut parts = input.split(" -> ").into_iter();

        // Parse start
        let start: Point = if let Some(start_part) = parts.next() {
            if let Ok(x) = Point::from_input(start_part.to_string()) {
                x
            } else {
                return Err(ParseError(input));
            }
        } else {
            return Err(ParseError(input));
        };

        // Parse end
        let end: Point = if let Some(end_part) = parts.next() {
            if let Ok(x) = Point::from_input(end_part.to_string()) {
                x
            } else {
                return Err(ParseError(input));
            }
        } else {
            return Err(ParseError(input));
        };

        Ok(Line { start, end })
    }

    /// Determines if the line is horizontal.
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// Determines if the line is vertical.
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// Format the line to a string.
    fn stringify(&self) -> String {
        format!("{} -> {}", self.start.stringify(), self.end.stringify())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{Point, Line};

    #[test]
    fn should_parse_point_from_input() {
        let input = "3,4".to_string();
        let expected = Ok(Point::new(3, 4));
        let actual = Point::from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_line_from_input() {
        let input = "3,4 -> 1,4".to_string();
        let expected = Ok(Line::new(Point::new(3, 4), Point::new(1, 4)));
        let actual = Line::from_input(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_horizontal_line_true() {
        let line = Line::new(Point::new(2, 4), Point::new(9, 4));
        let actual = line.is_horizontal();

        assert_eq!(actual, true);
    }

    #[test]
    fn should_determine_horizontal_line_false() {
        let line = Line::new(Point::new(3, 2), Point::new(1, 4));
        let actual = line.is_horizontal();

        assert_eq!(actual, false);
    }

    #[test]
    fn should_determine_vertical_line_true() {
        let line = Line::new(Point::new(2, 4), Point::new(2, 6));
        let actual = line.is_vertical();

        assert_eq!(actual, true);
    }

    #[test]
    fn should_determine_vertical_line_false() {
        let line = Line::new(Point::new(3, 4), Point::new(1, 4));
        let actual = line.is_vertical();

        assert_eq!(actual, false);
    }
}
