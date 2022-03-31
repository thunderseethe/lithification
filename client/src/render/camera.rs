use std::f32::consts::FRAC_PI_2;

use nalgebra as na;

pub struct Camera {
    pub position: na::Point3<f32>,
    //up: na::Vector3<f32>,
    yaw: f32,
    pitch: f32,
}

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

impl Camera {
    /*fn build_view_projection_matrix(&self) -> nalgebra::Matrix4<f32> {
        let opengl_to_wgpu_matrix: nalgebra::Matrix4<f32> =  
            [[1.0, 0.0, 0.0, 0.0], 
             [0.0, 1.0, 0.0, 0.0], 
             [0.0, 0.0, 0.5, 0.0],
             [0.0, 0.0, 0.5, 1.0]].into();

        let view = nalgebra::Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        /*let proj = nalgebra::Matrix4::new_perspective(
            self.aspect, 
            self.fovy, 
            self.znear, 
            self.zfar);*/
        return opengl_to_wgpu_matrix * proj * view;
    }*/

    pub fn new(
        position: impl Into<na::Point3<f32>>,
        yaw: f32,
        pitch: f32,
    ) -> Self {
        Self {
            position: position.into(),
            yaw,
            pitch
        }
    }

    pub fn calc_matrix(&self) -> na::Matrix4<f32> {
        let target = 
            self.position + 
            na::Vector3::<f32>::new(
            self.yaw.cos(),
            self.pitch.sin(),
            self.yaw.sin(),
            ).normalize();
        na::Matrix4::look_at_rh(
            &self.position, 
            &target,
            &na::Vector3::y_axis())
    }
}

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> na::Matrix4<f32> {
        let opengl_to_wgpu_matrix: nalgebra::Matrix4<f32> =  
            [[1.0, 0.0, 0.0, 0.0], 
             [0.0, 1.0, 0.0, 0.0], 
             [0.0, 0.0, 0.5, 0.0],
             [0.0, 0.0, 0.5, 1.0]].into();

        opengl_to_wgpu_matrix * na::Matrix4::new_perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }

    /*pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        }
    }*/

    pub fn process_event(&mut self, msg: common::Message) {
        if msg.tag == "player_left" {
            self.amount_left = 1.0;
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                y: scroll,
                ..
            }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}
