use raylib::misc::AsF32;


pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn dist(&self) -> f32 {
        self.dist_from(0.0, 0.0)
    }

    fn dist_from<T1: AsF32, T2: AsF32>(&self, x: T1, y: T2) -> f32 {
        let xx = x.as_f32() - self.x();
        let yy = y.as_f32() - self.y();
        (xx.powf(2.0) + yy.powf(2.0)).sqrt()
    }

    fn dist_from_point<T: Point>(&self, point: &T) -> f32 {
        self.dist_from(point.x(), point.y())
    }

    fn theta(&self) -> f32 {
        (self.y() / self.x()).atan()
    }
}


pub trait MovablePoint {
    fn x_ref(&mut self) -> &mut f32;
    fn y_ref(&mut self) -> &mut f32;

    fn move_to<T1: AsF32, T2: AsF32>(&mut self, x: T1, y: T2) {
        *self.x_ref() = x.as_f32();
        *self.y_ref() = y.as_f32();
    }

    fn move_to_point<T: Point>(&mut self, point: &T) {
        self.move_to(point.x(), point.y())
    }
}