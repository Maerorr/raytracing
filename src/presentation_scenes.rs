use crate::{color::Color, geometry::{Sphere, Surface}, light::{Light, RectangleAreaLight}, math::Vector, scene::Scene, BLACK_MAT, BLUE_MAT, GREEN_MAT, MIRROR_MAT, RED_MAT, WHITE_MAT};

pub fn shading_scene() -> Scene {
    let mut scene = Scene::new();

    let floor = Surface::new_vw(
        Vector::new(0.0, -300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, 1.0, 0.0)
    );
    scene.add_primitive(Box::new(floor), BLUE_MAT);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -500.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(back_wall), RED_MAT);

    let left_wall = Surface::new_vw(
        Vector::new(-300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(1.0, 0.0, 0.0)
    );
    scene.add_primitive(Box::new(left_wall), RED_MAT);

    let ceiling = Surface::new_vw(
        Vector::new(0.0, 300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, -1.0, 0.0)
    );
    scene.add_primitive(Box::new(ceiling), BLUE_MAT);

    let sphere = Sphere::new(Vector::new(-125.0, -220.0, -100.0), 50.0);
    scene.add_primitive(Box::new(sphere), BLUE_MAT);

    let sphere2 = Sphere::new(Vector::new(-125.0, 100.0, -200.0), 50.0);
    scene.add_primitive(Box::new(sphere2), RED_MAT);

    let sphere3 = Sphere::new(Vector::new(0.0, -200.0, -240.0), 50.0);
    scene.add_primitive(Box::new(sphere3), GREEN_MAT);

    let ambient = Light::new_ambient(Color::white(), 0.11);
    scene.add_light(ambient);

    let area_light = RectangleAreaLight::new(
        Vector::new(380.0, 0.0, -150.0),
        Color::white(),
        (60.0, 0.003, 0.0002),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        70.0,
        70.0,
        6.0,
    );
    scene.add_lights(area_light.get_lights());
    scene
}

pub fn reflection_refraction_scene() -> Scene {
    let mut scene = Scene::new();

    let mut mat_change = false;
    // cheater a checkerboard floor made of black and white surfaces
    let y = -(512.0 / 2.0);
    for x in 0..13 {
        for z in 0..13 {
            let mat = if mat_change {
                WHITE_MAT
            } else {
                BLACK_MAT
            };
            mat_change = !mat_change;
            let surface = Surface::new_vw(
                Vector::new(-300.0 + x as f32 * 50.0, y, -z as f32 * 50.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Some((-25.0, 25.0)),
                Some((-25.0, 25.0)),
                Vector::new(0.0, 1.0, 0.0),
            );
            scene.add_primitive(Box::new(surface), mat);
        }
    }
    
    // create left, right and back wall
    let left_wall = Surface::new_vw(
        Vector::new(-300.0, 0.0, -(12.0 * 50.0) / 2.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-320.0, 320.0)),
        Some((-250.0, 250.0)),
        Vector::new(1.0, 0.0, 0.0),
    );
    scene.add_primitive(Box::new(left_wall), RED_MAT);

    let right_wall = Surface::new_vw(
        Vector::new(300.0, 0.0, -(12.0 * 50.0) / 2.0),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-320.0, 320.0)),
        Some((-250.0, 250.0)),
        Vector::new(-1.0, 0.0, 0.0),
    );
    scene.add_primitive(Box::new(right_wall), BLUE_MAT);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -(12.0 * 50.0)),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-320.0, 320.0)),
        Some((-250.0, 250.0)),
        Vector::new(0.0, 0.0, 1.0),
    );
    scene.add_primitive(Box::new(back_wall), WHITE_MAT);

    let ball = Sphere::new(Vector::new(0.0, -50.0, -300.0), 75.0);
    scene.add_primitive(Box::new(ball), MIRROR_MAT);

    let amb = Light::new_ambient(Color::white(), 0.1);
    scene.add_light(amb);

    let point = Light::new_point(Vector::new(150.0, 0.0, -50.0), Color::white(), (1.0, 0.001, 0.00001));
    scene.add_light(point);

    let area_light = RectangleAreaLight::new(
        Vector::new(150.0, 0.0, -50.0),
        Color::white(),
        (60.0, 0.003, 0.0002),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        80.0,
        80.0,
        6.0,
    );
    scene.add_lights(area_light.get_lights());

    scene
    
}