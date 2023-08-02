pub enum Geometry {
    Rect(f32, f32, f32, f32), // x, y, width, height
    Circle(f32, f32, f32) // x, y, radius
}

pub trait Collidable {
    fn hitbox(&self) -> Geometry;
    fn overlaps(&self, other: &dyn Collidable) -> bool {
        match (self.hitbox(), other.hitbox()) {
            (Geometry::Circle(x1, y1, r1), Geometry::Circle(x2, y2, r2)) => {
                let dist_squared = (x1 - x2).powf(2.) + (y1 - y2).powf(2.);
                dist_squared < (r1 + r2).powf(2.)
            },
            (Geometry::Rect(x1, y1, w1, h1), Geometry::Rect(x2, y2, w2, h2)) => {
                x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
            },
            (Geometry::Circle(_, _, _), Geometry::Rect(_, _, _, _)) | (Geometry::Rect(_, _, _, _), Geometry::Circle(_, _, _)) => {
                false
            }
        }
    }
}
