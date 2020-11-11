use crate::Matrix;

pub struct Camera {
    pub hsize: u16,
    pub vsize: u16,
    pub half_width: f64,
    pub half_height: f64,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: u16, vsize: u16, field_of_view: f64) -> Self {
        let view_units = (field_of_view / 2.0).tan() * 2.0;
        let max_dimension = hsize.max(vsize) as f64;

        let pixel_size = view_units / max_dimension;

        let half_width = hsize as f64 * pixel_size / 2.0;
        let half_height = vsize as f64 * pixel_size / 2.0;

        // Original formula
        //
        // let half_view = (field_of_view / 2.0).tan();
        // let aspect = hsize as f64 / vsize as f64;
        // let (half_width, half_height) = if aspect >= 1.0 {
        //     (half_view, half_view / aspect)
        // } else {
        //     (half_view * aspect, half_view)
        // };
        // let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            field_of_view,
            transform: Matrix::identity(4),
            pixel_size,
        }
    }
}
