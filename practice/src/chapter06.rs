use std::{f64::consts::PI, sync::Mutex};
use std::{
    io::{self, Write},
    sync::Arc,
};

use library::{
    interface::Image,
    math::{Matrix, Tuple},
    properties::FlatPattern,
    properties::COLOR_BLACK,
    space::{Intersection, PointLight, Ray, Shape, Sphere},
    Axis,
};
use sdl2_interface::Sdl2Interface;

use rayon::prelude::*;

fn hit<'a>(ray: &Ray, sphere: &'a Sphere) -> Option<Intersection<'a>> {
    // At this stage, shapes always returned ordered hits, so we can use the first.
    //
    sphere.intersections(ray).get(0).cloned()
}

pub fn practice() {
    // The wall matches the display; for simplicity, the "wall" concept represents both.
    //
    const WALL_SIZE: u16 = 100;
    let eye_z = -50.0;
    let wall_z = 50.0;
    let light_position = (-20, 30, -50);

    let (origin_x, origin_y) = ((WALL_SIZE / 2) as i16, (WALL_SIZE / 2) as i16);

    let mut sphere = Sphere::default();
    sphere.material.pattern = Box::new(FlatPattern::new(1, 0.2, 1));
    sphere.transform = Matrix::translation(10, 0, 0)
        * &Matrix::rotation(Axis::Z, -PI / 4.0)
        * &Matrix::scaling(6.25, 12.5, 12.5);

    let sphere = Arc::new(sphere);

    let light = PointLight::new(light_position, (1, 1, 1));

    let eye_position = Tuple::point(0, 0, eye_z);

    let mut pixels_buffer = vec![vec![COLOR_BLACK; WALL_SIZE as usize]; WALL_SIZE as usize];
    let pixels_buffer_mtx = Mutex::new(&mut pixels_buffer);

    // buffer_y/x are just for convenience.
    //
    (-origin_y..origin_y)
        .into_par_iter()
        .enumerate()
        .for_each(|(buffer_y, interface_y)| {
            let mut row_buffer = vec![COLOR_BLACK; WALL_SIZE as usize];

            for (buffer_x, interface_x) in (-origin_x..origin_x).enumerate() {
                let eye_ray_direction =
                    Tuple::vector(interface_x as f64, interface_y as f64, wall_z - eye_z)
                        .normalize();

                let eye_ray = Ray {
                    origin: eye_position,
                    direction: eye_ray_direction,
                };

                if let Some(hit) = hit(&eye_ray, &sphere) {
                    let hit_point = eye_ray.position(hit.t);
                    let intersection = Intersection {
                        t: 0.0,
                        uv: None,
                        object: &Sphere::default(), // phony
                    };
                    let hit_normal = sphere.normal(&hit_point, &intersection);

                    let light_color = sphere.lighting(
                        &light,
                        &hit_point,
                        &-eye_ray.direction,
                        &hit_normal,
                        false,
                    );

                    row_buffer[buffer_x] = light_color;
                };
            }

            let mut pixels_buffer = pixels_buffer_mtx.lock().unwrap();
            pixels_buffer[buffer_y] = row_buffer;

            print!(".");
            io::stdout().flush().unwrap(); // makes sure that the output is flushed, since O/S generally do it per-line.
        });

    println!();

    let mut interface: Sdl2Interface =
        Sdl2Interface::from_pixels(pixels_buffer, WALL_SIZE, WALL_SIZE);

    interface.wait_keypress();
}
