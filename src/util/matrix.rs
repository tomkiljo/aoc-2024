use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self {
            Direction::North => "N",
            Direction::NorthEast => "NE",
            Direction::East => "E",
            Direction::SouthEast => "SE",
            Direction::South => "S",
            Direction::SouthWest => "SW",
            Direction::West => "W",
            Direction::NorthWest => "NW",
        };
        write!(f, "{}", dir)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

#[allow(dead_code)]
impl Point2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Point2D::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Point2D::new(self.x - other.x, self.y - other.y)
    }

    pub fn mul(self, scalar: usize) -> Self {
        Point2D::new(self.x * scalar, self.y * scalar)
    }

    pub fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Point2D::new(self.x, self.y.wrapping_sub(1)),
            Direction::NorthEast => Point2D::new(self.x.wrapping_add(1), self.y.wrapping_sub(1)),
            Direction::East => Point2D::new(self.x.wrapping_add(1), self.y),
            Direction::SouthEast => Point2D::new(self.x.wrapping_add(1), self.y.wrapping_add(1)),
            Direction::South => Point2D::new(self.x, self.y.wrapping_add(1)),
            Direction::SouthWest => Point2D::new(self.x.wrapping_sub(1), self.y.wrapping_add(1)),
            Direction::West => Point2D::new(self.x.wrapping_sub(1), self.y),
            Direction::NorthWest => Point2D::new(self.x.wrapping_sub(1), self.y.wrapping_sub(1)),
        }
    }
}

pub trait MatrixOps<'a, T> {
    fn at(&self, x: usize, y: usize) -> Option<&T>;
    fn at_point(&self, point: &Point2D) -> Option<&T> {
        self.at(point.x, point.y)
    }
    fn iter_points(&'a self) -> impl Iterator<Item = (Point2D, Option<&'a T>)>
    where
        T: 'a;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl<'a, T> MatrixOps<'a, T> for Vec<Vec<T>> {
    fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }
    fn iter_points(&'a self) -> impl Iterator<Item = (Point2D, Option<&'a T>)> {
        self.iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (x, y, cell)))
            .map(|(x, y, cell)| (Point2D::new(x, y), Some(cell)))
    }
    fn width(&self) -> usize {
        self.get(0).map_or(0, |row| row.len())
    }
    fn height(&self) -> usize {
        self.len()
    }
}
