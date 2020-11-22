use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};
use crate::math::Matrix;

#[derive(Debug, SmartDefault)]
pub struct StripePattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
}

impl Pattern for StripePattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        if pattern_point.x.floor() as i32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
