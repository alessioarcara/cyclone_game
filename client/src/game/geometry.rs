pub enum Geometry {
    Rect(f32, f32, f32, f32), // x, y, width, height
    Circle(f32, f32, f32) // x, y, radius
}

pub trait Collidable {
    fn is_colliding(&self) -> bool;
    fn set_is_colliding(&mut self, is_colliding: bool);
    fn hitbox(&self) -> Vec<Geometry>;
    fn overlaps(&self, other_hitbox: &Vec<Geometry>) -> bool {
        self.hitbox().iter().any(|geometry| {
            other_hitbox.iter().any(|other_geometry| {
                match (geometry, other_geometry) {
                    (Geometry::Circle(x1, y1, r1), Geometry::Circle(x2, y2, r2)) => {
                        let dist_squared = (x1 - x2).powf(2.) + (y1 - y2).powf(2.);
                        dist_squared < (r1 + r2).powf(2.)
                    },
                    (Geometry::Rect(x1, y1, w1, h1), Geometry::Rect(x2, y2, w2, h2)) => {
                        *x1 < x2 + w2 && x1 + w1 > *x2 && *y1 < y2 + h2 && y1 + h1 > *y2
                    },
                    (Geometry::Circle(_, _, _), Geometry::Rect(_, _, _, _)) | (Geometry::Rect(_, _, _, _), Geometry::Circle(_, _, _)) => {
                        false
                    },
                }
            })
        })
    }
}
