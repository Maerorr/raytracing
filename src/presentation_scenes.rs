use crate::{color::Color, geometry::{Sphere, Surface}, light::{Light, RectangleAreaLight}, material::Material, math::{as_radians, Vector}, scene::Scene, BLACK_MAT, BLUE_MAT, GLASS_MAT, GREEN_MAT, MIRROR_MAT, RED_MAT, WHITE_MAT};
use image::{io::Reader as ImageReader, ImageBuffer};

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

    let sphere_roughness00 = Sphere::new(Vector::new(-300.0, -300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness00), mat_count);
    let material_roughness00 = Material::new_pbr(Color::red(), 0.0, 0.3, 1.3, 0.0, as_radians(30.0));
    materials.push(material_roughness00);
    mat_count += 1;

    let sphere_roughness025 = Sphere::new(Vector::new(300.0, -300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness025), mat_count);
    let material_roughness01 = Material::new_pbr(Color::red(), 0.0, 0.3, 1.3, 0.0, as_radians(30.0));
    materials.push(material_roughness01);
    mat_count += 1;

    let sphere_roughness05 = Sphere::new(Vector::new(-300.0, 300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness05), mat_count);
    let material_roughness05 = Material::new_pbr(Color::red(), 0.0, 0.3, 1.3, 0.0, as_radians(30.0));
    materials.push(material_roughness05);
    mat_count += 1;

    let sphere_roughness075 = Sphere::new(Vector::new(300.0, 300.0, -900.0), 200.0);
    scene.add_primitive(Box::new(sphere_roughness075), mat_count);
    let material_roughness075 = Material::new_pbr(Color::red(), 0.0, 0.3, 1.3, 0.0, as_radians(30.0));
    materials.push(material_roughness075);
    mat_count += 1;

    let pos = Vector::new(0.0, 0.0, -500.0);
    let sphere = Sphere::new(pos, 100.0);
    scene.add_primitive(Box::new(sphere), mat_count);
    let material = Material::new_pbr(Color::green(), 1.0, 0.4, 1.3, 0.9, as_radians(30.0));
    materials.push(material);
    mat_count += 1;

    let blue_mat = Material::new_pbr(
        Color::blue(),
        0.1,
        0.5,
        1.3, 0.0, as_radians(30.0)
    );
    let red_mat = Material::new_pbr(
        Color::red(),
        0.4,
        0.2,
        1.3, 0.0, as_radians(30.0)
    );
    let green_mat = Material::new_pbr(
        Color::green(),
        0.1,
        0.8,
        1.3, 0.0, as_radians(30.0)
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

    //let ambient = Light::new_ambient(Color::white(), 0.01);
    //scene.add_light(ambient);

    // let area_light = RectangleAreaLight::new(
    //     Vector::new(0.0, 0.0, 0.0),
    //     Color::white(),
    //     (50.0, 0.002, 0.0001),
    //     Vector::new(0.0, 1.0, 0.0),
    //     Vector::new(1.0, 0.0, 0.0),
    //     300.0,
    //     300.0,
    //     3.0,
    // );
    // scene.add_lights(area_light.get_lights());
    let point = Light::new_point(Vector::new(300.0, 0.0, 0.0), Color::white(), (0.1, 0.000001, 0.000004));
    scene.add_light(point);

    (scene, materials)
}

pub fn texture_test() -> (Scene, Vec<Material>) {
    let mut scene = Scene::new();
    let mut materials = Vec::new();

    let white_mat = Material::new_pbr(
        Color::white(),
        0.1,
        0.5,
        1.3, 0.0, as_radians(30.0)
    );
    materials.push(white_mat);

    let tex_quad = Surface::new_vw(
        Vector::new(0.0, 0.0, -1000.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(tex_quad), 0);

    let albedo_texture = ImageReader::open("res/albedo.png").unwrap().decode().unwrap().into_rgb8();
    let metal_texture = ImageReader::open("res/metal.png").unwrap().decode().unwrap().into_rgb8();
    let roughness_texture = ImageReader::open("res/rough.png").unwrap().decode().unwrap().into_rgb8();

    let textured_mat = Material::new_textured_pbr(
        albedo_texture,
        metal_texture,
        roughness_texture
    );
    materials.push(textured_mat);

    let tex_quad = Surface::new_vw(
        Vector::new(0.0, 0.0, -500.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-400.0, 400.0)),
        Some((-400.0, 400.0)),
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(tex_quad), 1);

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

    (scene, materials)
}

pub fn full_pbr_scene() -> (Scene, Vec<Material>) {
    let mut scene = Scene::new();
    let mut materials = Vec::new();

    let mut red = Color::red();
    //red.add_random_offset(0.1);
    let rough_red1 = Material::new_pbr(red, 0.0, 0.1, 1.3, 0.0, as_radians(0.0));
    //red.add_random_offset(0.2);
    let rough_red2 = Material::new_pbr(red, 0.0, 0.3, 1.3, 0.0, as_radians(0.0));
    //red.add_random_offset(0.3);
    let rough_red3 = Material::new_pbr(red, 0.0, 0.6, 1.3, 0.0, as_radians(0.0));
    //red.add_random_offset(0.4);
    let rough_red4 = Material::new_pbr(red, 0.0, 0.9, 1.3, 0.0, as_radians(0.0));

    let green = Color::green();
    let metal_isotropic1 = Material::new_pbr(green, 0.95, 0.1, 1.3, 0.0, as_radians(0.0));
    let metal_isotropic2 = Material::new_pbr(green, 0.95, 0.3, 1.3, 0.0, as_radians(0.0));
    let metal_isotropic3 = Material::new_pbr(green, 0.95, 0.6, 1.3, 0.0, as_radians(0.0));
    let metal_isotropic4 = Material::new_pbr(green, 0.95, 0.9, 1.3, 0.0, as_radians(0.0));

    let blue = Color::blue();
    let metal_anisotropic1 = Material::new_pbr(blue, 0.99, 0.99, 1.3, 1.0, as_radians(15.0));
    let metal_anisotropic2 = Material::new_pbr(blue, 0.99, 0.99, 1.3, 1.0, as_radians(30.0));
    let metal_anisotropic3 = Material::new_pbr(blue, 0.99, 0.99, 1.3, 1.0, as_radians(45.0));
    let metal_anisotropic4 = Material::new_pbr(blue, 0.99, 0.99, 1.3, 1.0, as_radians(60.0));
    let metal_anisotropic5 = Material::new_pbr(blue, 0.99, 0.99, 1.3, 1.0, as_radians(75.0));

    let walls_white = Material::new_pbr(Color::white(), 0.99, 0.9, 1.3, 0.99, as_radians(15.0));

    let floor = Surface::new_vw(
        Vector::new(0.0, -900.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, 1.0, 0.0)
    );
    materials.push(walls_white);
    scene.add_primitive(Box::new(floor), 0);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -3000.0),
        Vector::new(-1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0)
    );
    scene.add_primitive(Box::new(back_wall), 0);

    materials.push(rough_red1);
    materials.push(rough_red2);
    materials.push(rough_red3);
    materials.push(rough_red4);

    // balls
    let sphere1 = Sphere::new(Vector::new(-1000.0, -600.0, -500.0), 200.0);
    let sphere2 = Sphere::new(Vector::new(-500.0, -600.0, -500.0), 200.0);
    let sphere3 = Sphere::new(Vector::new(0.0, -600.0, -500.0), 200.0);
    let sphere4 = Sphere::new(Vector::new(500.0, -600.0, -500.0), 200.0);
    scene.add_primitive(Box::new(sphere1), 1);
    scene.add_primitive(Box::new(sphere2), 2);
    scene.add_primitive(Box::new(sphere3), 3);
    scene.add_primitive(Box::new(sphere4), 4);

    materials.push(metal_isotropic1);
    materials.push(metal_isotropic2);
    materials.push(metal_isotropic3);
    materials.push(metal_isotropic4);

    let sphere1 = Sphere::new(Vector::new(-800.0, -100.0, -500.0), 200.0);
    let sphere2 = Sphere::new(Vector::new(-300.0, -100.0, -500.0), 200.0);
    let sphere3 = Sphere::new(Vector::new(200.0, -100.0, -500.0), 200.0);
    let sphere4 = Sphere::new(Vector::new(800.0, -100.0, -500.0), 200.0);
    scene.add_primitive(Box::new(sphere1), 5);
    scene.add_primitive(Box::new(sphere2), 6);
    scene.add_primitive(Box::new(sphere3), 7);
    scene.add_primitive(Box::new(sphere4), 8);

    materials.push(metal_anisotropic1);
    materials.push(metal_anisotropic2);
    materials.push(metal_anisotropic3);
    materials.push(metal_anisotropic4);
    materials.push(metal_anisotropic5);

    let sphere1 = Sphere::new(Vector::new(-1000.0, 300.0, -500.0), 200.0);
    let sphere2 = Sphere::new(Vector::new(-500.0, 300.0, -500.0), 200.0);
    let sphere3 = Sphere::new(Vector::new(0.0, 300.0, -500.0), 200.0);
    let sphere4 = Sphere::new(Vector::new(500.0, 300.0, -500.0), 200.0);
    let sphere5 = Sphere::new(Vector::new(1000.0, 300.0, -500.0), 200.0);

    scene.add_primitive(Box::new(sphere1), 9);
    scene.add_primitive(Box::new(sphere2), 10);
    scene.add_primitive(Box::new(sphere3), 11);
    scene.add_primitive(Box::new(sphere4), 12);
    scene.add_primitive(Box::new(sphere5), 13);

    let ambient = Light::new_ambient(Color::white(), 0.05);
    scene.add_light(ambient);
    
    let area_light = RectangleAreaLight::new(
        Vector::new(0.0, 0.0, 400.0),
        Color::white(),
        (50.0, 0.002, 0.0001),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        50.0,
        50.0,
        8.0,
    );
    scene.add_lights(area_light.get_lights());

    // let point = Light::new_point(Vector::new(0.0, 0.0, 500.0), Color::white(), (1.0, 0.000001, 0.000001));
    // scene.add_light(point);

    (scene, materials)
}