use raylib::misc::AsF32;
use raylib::prelude::*;


pub struct GameCamera {
    pub cam: Camera2D,
    default_zoom: f32,
    default_rotation: f32,
}


impl GameCamera {
    pub fn new() -> Self {
        let default_rotation: f32 = 0.0;
        let default_zoom: f32 = 1.0;
        GameCamera {
            default_rotation,
            default_zoom,
            cam: Camera2D{
                target: rvec2(0, 0),
                offset: rvec2(0, 0),
                rotation: default_rotation,
                zoom: default_zoom,
            },
        }
    }

    pub fn rotate_to<T: AsF32>(&mut self, r: T) {
        self.cam.rotation = r.as_f32();
    }

    pub fn zoom_to<T: AsF32>(&mut self, z: T) {
        self.cam.zoom = z.as_f32();
    }

    pub fn target_at<T1: AsF32, T2: AsF32>(&mut self, x: T1, y: T2) {
        self.cam.target = rvec2(x, y);
    }

    pub fn change_offset_to<T1: AsF32, T2: AsF32>(&mut self, x: T1, y: T2) {
        self.cam.offset = rvec2(x, y);
    }

    pub fn restore_zoom(&mut self) {
        self.cam.zoom = self.default_zoom;
    }

    pub fn restore_rotation(&mut self) {
        self.cam.rotation = self.default_rotation;
    }

    pub fn increase_offset_by<T1: AsF32, T2: AsF32>(&mut self, x: T1, y: T2) {
        let (ox, oy) = (self.cam.offset.x, self.cam.offset.y);
        self.change_offset_to(ox + x.as_f32(), oy + y.as_f32());
    }
}
