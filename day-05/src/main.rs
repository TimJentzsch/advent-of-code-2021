#[derive(Debug, PartialEq)]
struct ParseError(String);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
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
        let x: usize = if let Some(x_part) = parts.next() {
            if let Ok(x) = x_part.parse() {
                x
            } else {
                return Err(ParseError(input));
            }
        } else {
            return Err(ParseError(input));
        };

        // Parse y
        let y: usize = if let Some(y_part) = parts.next() {
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

    /// Get the points on the given line.
    /// 
    /// Currently only works for vertical and horizontal lines.
    fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        if self.is_vertical() {
            for y in self.start.y.min(self.end.y)..(self.start.y.max(self.end.y) + 1) {
                points.push(Point::new(self.start.x, y));
            }
        } else if self.is_horizontal() {
            for x in self.start.x.min(self.end.x)..(self.start.x.max(self.end.x) + 1) {
                points.push(Point::new(x, self.start.y));
            }
        }

        points
    }

    /// Format the line to a string.
    fn stringify(&self) -> String {
        format!("{} -> {}", self.start.stringify(), self.end.stringify())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Diagram<const R: usize, const C: usize> {
    values: [[usize; R]; C],
}

impl<const R: usize, const C: usize> Diagram<R, C> {
    /// Create a new, empty diagram.
    fn new() -> Diagram<R, C> {
        Diagram { values: [[0usize; R]; C]}
    }

    /// Add a point to the diagram.
    fn add_point(&mut self, point: Point) {
        self.values[point.y][point.x] += 1;
    }

    /// Add a line to the diagram.
    fn add_line(&mut self, line: Line) {
        for point in line.points() {
            self.add_point(point);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::{Point, Line, Diagram};

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

    #[test]
    fn should_determine_points_on_horizontal_line() {
        let line = Line::new(Point::new(9, 7), Point::new(7, 7));
        let expected = vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)];
        let actual = line.points();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_determine_points_on_vertical_line() {
        let line = Line::new(Point::new(1, 1), Point::new(1, 3));
        let expected = vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)];
        let actual = line.points();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_add_point_to_diagram() {
        let point = Point::new(1, 2);
        let expected = Diagram {
            values: [
                [0, 0, 0],
                [0, 0, 0],
                [0, 1, 0],
            ],
        };
        
        let mut actual = Diagram::<3, 3>::new();
        actual.add_point(point);

        assert_eq!(actual, expected);
    }

    

    #[test]
    fn should_add_line_to_diagram() {
        let line = Line::new(Point::new(1, 2), Point::new(1, 0));
        let expected = Diagram {
            values: [
                [0, 1, 0],
                [0, 1, 0],
                [0, 1, 0],
            ],
        };
        
        let mut actual = Diagram::<3, 3>::new();
        actual.add_line(line);

        assert_eq!(actual, expected);
    }
}
