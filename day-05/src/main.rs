use std::{fs, thread};

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

    fn max_x(&self) -> usize {
        self.start.x.max(self.end.x)
    }

    fn max_y(&self) -> usize {
        self.start.y.max(self.end.y)
    }

    fn min_x(&self) -> usize {
        self.start.x.min(self.end.x)
    }

    fn min_y(&self) -> usize {
        self.start.y.min(self.end.y)
    }

    /// Determines if the line is horizontal.
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// Determines if the line is vertical.
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// Determines if the line is diagonal.
    fn is_diagonal(&self) -> bool {
        (self.max_x() - self.min_x()) == (self.max_y() - self.min_y())
    }

    /// Get the points on the given line.
    ///
    /// Currently only works for vertical and horizontal lines.
    fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        if self.is_vertical() {
            for y in self.min_y()..(self.max_y() + 1) {
                points.push(Point::new(self.start.x, y));
            }
        } else if self.is_horizontal() {
            for x in self.min_x()..(self.max_x() + 1) {
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
        Diagram {
            values: [[0usize; R]; C],
        }
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

    /// Count the number of points where their value exceeds the threshold.
    ///
    /// This means lines overlap at that many points.
    fn count_points(&self, threshold: usize) -> usize {
        let mut count = 0;

        for row in 0..R {
            for col in 0..C {
                if self.values[row][col] >= threshold {
                    count += 1;
                }
            }
        }

        count
    }

    /// Create a string from the diagram.
    fn stringify(&self) -> String {
        let mut output = "".to_string();

        for row in 0..R {
            for col in 0..C {
                let value = self.values[row][col];

                if value == 0 {
                    output += ".";
                } else {
                    output += &value.to_string();
                }
            }

            output += "\n";
        }

        output
    }
}

const STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    // Run the code in a new thread to prevent stack overflows
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}

fn run() {
    // Read the input file
    println!("Starting");
    let filename = "./input/input.txt";
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = parse_input(input);
    println!("Creating diagram");
    let mut diagram = Box::new(Diagram::<1000, 1000>::new());
    println!("Diagram created");

    // Draw in all horizontal or vertical lines
    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            diagram.add_line(line);
        }
    }

    let count = diagram.count_points(2);
    println!("At least two lines overlap at {} points!", count);
}

fn parse_input(input: String) -> Vec<Line> {
    let line_strs = input.split("\n").into_iter();

    line_strs
        // Parse the line
        .map(|line_str| Line::from_input(line_str.to_string()))
        // Filter out invalid lines
        .filter(|res| match res {
            Ok(_) => true,
            Err(_) => false,
        })
        // Extract the result
        .map(|res| res.unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{Diagram, Line, Point, parse_input};

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
    fn should_determine_diagonal_line_true() {
        let line = Line::new(Point::new(9, 7), Point::new(7, 9));
        let actual = line.is_diagonal();

        assert_eq!(actual, true);
    }

    #[test]
    fn should_determine_diagonal_line_false() {
        let line = Line::new(Point::new(3, 4), Point::new(1, 4));
        let actual = line.is_diagonal();

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
            values: [[0, 0, 0], [0, 0, 0], [0, 1, 0]],
        };

        let mut actual = Diagram::<3, 3>::new();
        actual.add_point(point);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_add_line_to_diagram() {
        let line = Line::new(Point::new(1, 2), Point::new(1, 0));
        let expected = Diagram {
            values: [[0, 1, 0], [0, 1, 0], [0, 1, 0]],
        };

        let mut actual = Diagram::<3, 3>::new();
        actual.add_line(line);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_count_diagram_points() {
        let line1 = Line::new(Point::new(1, 2), Point::new(1, 0));
        let line2 = Line::new(Point::new(1, 1), Point::new(2, 1));
        let mut diagram = Diagram::<3, 3>::new();
        diagram.add_line(line1);
        diagram.add_line(line2);

        let actual = diagram.count_points(2);

        assert_eq!(actual, 1);
    }

    #[test]
    fn should_parse_input_lines() {
        let input = "0,9 -> 5,9\n8,0 -> 0,8\n".to_string();
        let expected = vec![
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
        ];
        let actual = parse_input(input);

        assert_eq!(actual, expected);
    }
}
