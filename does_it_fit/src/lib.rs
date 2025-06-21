pub mod areas_volumes;
pub use areas_volumes::GeometricalShapes;
pub use areas_volumes::GeometricalVolumes;
pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let rec_area= areas_volumes::rectangle_area(x,y );
    match kind {
        GeometricalShapes::Circle=>areas_volumes::circle_area(a) as usize * times<= rec_area,
        GeometricalShapes::Rectangle=>areas_volumes::rectangle_area(a, b) as usize * times<=rec_area,
        GeometricalShapes::Square=>areas_volumes::square_area(a)as usize*times<= rec_area,
        GeometricalShapes::Triangle=>areas_volumes::triangle_area(a, b)as usize*times<=rec_area,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let rec_vol = x*y*z;
    match kind {
        GeometricalVolumes::Cone=>areas_volumes::cone_volume(a, b)as usize * times <= rec_vol,
        GeometricalVolumes::Cube=>areas_volumes::cube_volume(a)as usize*times<= rec_vol,
        GeometricalVolumes::Parallelepiped=>areas_volumes::parallelepiped_volume(a, b, c) as usize* times <= rec_vol,
        GeometricalVolumes::Sphere=>areas_volumes::sphere_volume(a)as usize*times<=rec_vol,
        GeometricalVolumes::TriangularPyramid=>areas_volumes::triangular_pyramid_volume(a as f64, b) as usize *times<= rec_vol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!(
        "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
        area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1))
    );
    println!(
        "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
        area_fit((5, 5),  GeometricalShapes::Triangle, 3, (5, 3))
    );
    println!(
        "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
        volume_fit((5, 5, 5),  GeometricalVolumes::Sphere, 3, (2, 0, 0))
    );
    println!(
        "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
        volume_fit((5, 7, 5),  areas_volumes::GeometricalVolumes::Parallelepiped, 1, (6, 7, 4))
    );
    }
}
