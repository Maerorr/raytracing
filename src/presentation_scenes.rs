use crate::{color::Color, geometry::{Sphere, Surface}, light::{Light, RectangleAreaLight}, material::Material, math::Vector, scene::Scene, BLACK_MAT, BLUE_MAT, GLASS_MAT, GREEN_MAT, MIRROR_MAT, RED_MAT, WHITE_MAT};

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
    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -500.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(back_wall), WHITE_MAT);

    let floor = Surface::new_vw(
        Vector::new(0.0, -300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, 1.0, 0.0)
    );
    scene.add_primitive(Box::new(floor), WHITE_MAT);

    let roof = Surface::new_vw(
        Vector::new(0.0, 300.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, -1.0, 0.0)
    );
    scene.add_primitive(Box::new(roof), WHITE_MAT);

    let right_wall = Surface::new_vw(
        Vector::new(300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(-1.0, 0.0, 0.0)
    );
    scene.add_primitive(Box::new(right_wall), BLUE_MAT);

    let left_wall = Surface::new_vw(
        Vector::new(-300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(1.0, 0.0, 0.0)
    );

    scene.add_primitive(Box::new(left_wall), RED_MAT);

    let mirror_sphere = Sphere::new(Vector::new(-100.0, -200.0, -250.0), 100.0);
    scene.add_primitive(Box::new(mirror_sphere), MIRROR_MAT);

    let glass_sphere = Sphere::new(Vector::new(100.0, -200.0, -150.0), 100.0);
    scene.add_primitive(Box::new(glass_sphere), GLASS_MAT);

    let ambient = Light::new_ambient(Color::white(), 0.05);
    scene.add_light(ambient);

    let area_light = RectangleAreaLight::new(
        Vector::new(0.0, 250.0, -150.0),
        Color::white(),
        (30.0, 0.002, 0.0003),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(1.0, 0.0, 0.0),
        50.0,
        50.0,
        6.0,
    );
    scene.add_lights(area_light.get_lights());
    
    scene
}

pub fn pbr_scene() -> (Scene, Vec<Material>) {
    let mut scene = Scene::new();
    let mut materials = Vec::new();
    let mut mat_count = 0;
    // for i in 0..2 {
    //     for j in 0..2 {
    //         let mut pos = Vector::new(-300.0 + i as f32 * 600.0, -300.0 + j as f32 * 600.0, -900.0);
    //         let sphere = Sphere::new(pos, 200.0);
    //         scene.add_primitive(Box::new(sphere), mat_count);
    //         let material = Material::new_pbr(Color::red(), 0.3, mat_count as f32 * 0.25);
    //         println!("created pbr material with roughness: {}", mat_count as f32 * 0.25);
    //         materials.push(material);
    //         mat_count += 1;
    //         let light_pos = Vector::new(-50.0 + i as f32 * 100.0, -50.0 + j as f32 * 100.0, -200.0);
    //         let point = Light::new_point(light_pos, Color::white(), (0.1, 0.001, 0.0004));
    //         scene.add_light(point);
    //     }
    // }

    let sphere_roughness00 = Sphere::new(Vector::new(-300.0, -300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness00), mat_count);
    let material_roughness00 = Material::new_pbr(Color::red(), 0.3, 0.0);
    materials.push(material_roughness00);
    mat_count += 1;

    let sphere_roughness025 = Sphere::new(Vector::new(300.0, -300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness025), mat_count);
    let material_roughness01 = Material::new_pbr(Color::red(), 0.3, 0.25);
    materials.push(material_roughness01);
    mat_count += 1;

    let sphere_roughness05 = Sphere::new(Vector::new(-300.0, 300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness05), mat_count + 2);
    let material_roughness05 = Material::new_pbr(Color::red(), 0.3, 0.5);
    materials.push(material_roughness05);
    mat_count += 1;

    let sphere_roughness075 = Sphere::new(Vector::new(300.0, 300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness075), mat_count + 3);
    let material_roughness075 = Material::new_pbr(Color::red(), 0.3, 0.75);
    materials.push(material_roughness075);
    mat_count += 1;

    // let light = Light::new_point(Vector::new(0.0, 0.0, -200.0), Color::white(), (1.0, 0.001, 0.00002));
    // scene.add_light(light);

    let pos = Vector::new(0.0, 0.0, -500.0);
    let sphere = Sphere::new(pos, 100.0);
    scene.add_primitive(Box::new(sphere), mat_count);
    let material = Material::new_pbr(Color::green(), 0.7, 0.5);
    materials.push(material);

    let blue_mat = Material::new_pbr(
        Color::blue(),
        0.1,
        0.5,
    );
    let red_mat = Material::new_pbr(
        Color::red(),
        0.4,
        0.2,
    );
    let green_mat = Material::new_pbr(
        Color::green(),
        0.1,
        0.8,
    );
    materials.push(blue_mat);
    materials.push(red_mat);
    materials.push(green_mat);

    let right_wall = Surface::new_vw(
        Vector::new(600.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(-1.0, 0.0, 0.0)
    );
    scene.add_primitive(Box::new(right_wall), mat_count);
    

    let left_wall = Surface::new_vw(
        Vector::new(-600.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(1.0, 0.0, 0.0)
    );
    mat_count += 1;
    scene.add_primitive(Box::new(left_wall), mat_count);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -1300.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    mat_count += 1;
    scene.add_primitive(Box::new(back_wall), mat_count);

    let ambient = Light::new_ambient(Color::white(), 0.05);
    scene.add_light(ambient);

    let area_light = RectangleAreaLight::new(
        Vector::new(0.0, 0.0, 0.0),
        Color::white(),
        (50.0, 0.002, 0.0001),
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(1.0, 0.0, 0.0),
        100.0,
        100.0,
        8.0,
    );
    scene.add_lights(area_light.get_lights());
    // let point = Light::new_point(Vector::new(0.0, 0.0, -500.0), Color::white(), (0.1, 0.000001, 0.000004));
    // scene.add_light(point);

    (scene, materials)
}