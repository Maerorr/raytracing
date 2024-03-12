use crate::line::Line;
use crate::object::Object;
use crate::quaternion::Quaternion;
use crate::raycasthit::RayCastHit;
use crate::vector::Vector;

pub struct Camera {
    pub line: Line,
    pub render_width: i32,
    pub render_height: i32,
    pub up: Vector,
    pub right: Vector,
    default: (Vector, Vector),
    rotation: Quaternion,
    debug: String,
    pub backface_culling: bool,
}

impl Camera {
    pub fn new(camera_position: Vector, camera_look_direction: Vector, width: i32, height: i32, up: Vector, right: Vector) -> Camera {
        Camera {
            line: Line::new(camera_position, camera_look_direction),
            render_width: width,
            render_height: height,
            default: (camera_position, camera_look_direction),
            up,
            right,
            rotation: Quaternion::identity(),
            debug: String::new(),
            backface_culling: false,
        }
    }

    pub fn render(&mut self, object: &Object) -> Vec<RayCastHit> {
        // THIS IS JUST TO ROTATE THE CAMERA ONCE PER RENDER WITHOUT IT SPINNING AROUND
        let mut l = self.line.clone();
        let mut point = self.line.point;
        point.rotate_by_quaternion(&self.rotation);
        l.point.rotate_by_quaternion(&self.rotation);
        l.direction.rotate_by_quaternion(&self.rotation);
        let mut up = self.up.clone();
        let mut right = self.right.clone();
        up.rotate_by_quaternion(&self.rotation);
        right.rotate_by_quaternion(&self.rotation);

        self.debug.clear();
        self.debug.push_str(&format!("Camera position: {}\n", l.point.to_string()));
        self.debug.push_str(&format!("Camera direction: {}\n", l.direction.to_string()));
        self.debug.push_str(&format!("Camera up: {}\n", up.to_string()));
        self.debug.push_str(&format!("Camera right: {}\n", right.to_string()));

        let mut hits: Vec<RayCastHit> = Vec::new();
        for i in (-self.render_height / 2)..(self.render_height / 2) {
            for j in (-self.render_width / 2)..(self.render_width / 2) {
                l.point = point + up * i as f64 + right * j as f64;
                let mut hit = l.intersection_object(&object, &l.point, &self.backface_culling);
                hit.pos_on_screen = (j, i);
                hits.push(hit);
            }
        }
        hits
    }

    pub fn set_camera_position(&mut self, v: &Vector) {
        self.line.point = *v;
    }

    pub fn set_camera_rotation(&mut self, q: &Quaternion) {
        self.rotation = *q;
    }

    pub fn set_as_default(&mut self) {
        self.default = (self.line.point, self.line.direction);
    }

    pub fn default(&mut self) {
        self.line.point = self.default.0;
        self.line.direction = self.default.1;
    }

    pub fn get_debug_info(&self) -> String {
        self.debug.clone()
    }
}