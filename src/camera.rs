use raylib::misc::AsF32;
use raylib::prelude::*;


pub struct GameCamera {
    pub cam: Camera2D,
    default_zoom: f32,
    default_rotation: f32,
}


impl GameCamera {
    pub fn from(target: Vector2, offset: Vector2) -> GameCamera {
        GameCamera {
            cam: Camera2D{
                target: target,
                offset: offset,
                rotation: 0.0,
                zoom: 1.0,
            }
        }
    }

    pub fn rotate_to<T: AsF32>(self, r: T) {
        self.cam.rotation = r;
    }

    pub fn zoom_to<T: AsF32>(self, z: T) {
        self.cam.zoom = z;
    }

    pub fn target_at<T: AsF32, U: AsF32>(self, x: T, y: U) {
        self.cam.target = rvec2(x, y);
    }

    pub fn change_offset_to<T: AsF32, U: AsF32>(self, x: T, y: U) {
        self.cam.offset = rvec2(x, y);
    }

    pub fn restore_zoom(self) {
        self.cam.zoom = self.default_zoom;
    }

    pub fn restore_rotation(self) {
        self.cam.rotation = self.default_rotation;
    }
}
