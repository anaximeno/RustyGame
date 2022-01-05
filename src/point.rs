pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn dist(&self) -> f32 {
        self.dist_from(0.0, 0.0)
    }

    fn dist_from(&self, x: f32, y: f32) -> f32 {
        ((x - self.x()).powf(2.0) + (y - self.y()).powf(2.0)).sqrt()
    }

    fn dist_from_point<T: Point>(&self, point: &T) -> f32 {
        self.dist_from(point.x(), point.y())
    }

    fn theta(&self) -> f32 {
        (self.y() / self.x()).atan()
    }
}
