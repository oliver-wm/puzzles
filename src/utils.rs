#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn collinear(p1: Point, p2: Point, p3: Point) -> bool {
        let a = p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y);
        if a == 0 {
            return true;
        }
        false
    }
}

impl TryInto<(usize, usize)> for Point {
    type Error = ();
    fn try_into(self) -> Result<(usize, usize), ()> {
        if self.x < 0 || self.y < 0 {
            return Err(());
        }
        Ok((self.x as usize, self.y as usize))
    }
}

impl From<&(usize, usize)> for Point {
    fn from(value: &(usize, usize)) -> Self {
        Point {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::Point;

    #[test]
    fn test_collinear() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 1, y: 4 };
        let p3 = Point { x: 1, y: 5 };

        assert_eq!(Point::collinear(p1, p2, p3), true);
        assert_eq!(Point::collinear(p2, p1, p3), true);
        assert_eq!(Point::collinear(p3, p2, p1), true);
        assert_eq!(Point::collinear(p2, p3, p1), true);
        assert_eq!(Point::collinear(p3, p1, p2), true);
    }
}
