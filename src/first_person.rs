// Original PistonDevelopers/camera_controllers license:
//
// The MIT License (MIT)
//
// Copyright (c) 2015 PistonDevelopers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A 3D first-person camera type.
//!
//! Adapted from PistonDevelopers/camera_controllers under the MIT license
//! https://github.com/PistonDevelopers/camera_controllers/blob/8edf8a464b57107683c2585bbdc55d9e46994783/src/first_person.rs

use vecmath::traits::{ Float, Radians };

use Camera;

bitflags!(pub struct Actions: u8 {
    const MOVE_FORWARD  = 0b00000001;
    const MOVE_BACKWARD = 0b00000010;
    const STRAFE_LEFT   = 0b00000100;
    const STRAFE_RIGHT  = 0b00001000;
    const FLY_UP        = 0b00010000;
    const FLY_DOWN      = 0b00100000;
    const MOVE_FASTER   = 0b01000000;
});

/// First person camera settings.
pub struct FirstPersonSettings<T=f32> {
    /// The horizontal movement speed.
    ///
    /// This is measured in units per second.
    pub speed_horizontal: T,
    /// The vertical movement speed.
    ///
    /// This is measured in units per second.
    pub speed_vertical: T,
    /// The horizontal mouse sensitivity.
    ///
    /// This is a multiplier applied to horizontal mouse movements.
    pub mouse_sensitivity_horizontal: T,
    /// The vertical mouse sensitivity.
    ///
    /// This is a multiplier applied to vertical mouse movements.
    pub mouse_sensitivity_vertical: T,
}

impl<T> Default for FirstPersonSettings<T>
    where T: Float
{
    /// Creates new first person camera settings with wasd defaults.
    fn default() -> FirstPersonSettings<T> {
        FirstPersonSettings {
            speed_horizontal: T::one(),
            speed_vertical: T::one(),
            mouse_sensitivity_horizontal: T::one(),
            mouse_sensitivity_vertical: T::one(),
        }
    }
}

/// Models a flying first person camera.
pub struct FirstPerson<T=f32> {
    /// The first person camera settings.
    pub settings: FirstPersonSettings<T>,
    /// The yaw angle (in radians).
    pub yaw: T,
    /// The pitch angle (in radians).
    pub pitch: T,
    /// The position of the camera.
    pub position: [T; 3],
    /// The velocity we are moving.
    pub velocity: T,
    /// The active actions.
    pub actions: Actions,
}

impl<T> FirstPerson<T>
    where T: Float
{
    /// Creates a new first person camera.
    pub fn new(
        position: [T; 3],
        settings: FirstPersonSettings<T>
    ) -> FirstPerson<T> {
        let _0: T = T::zero();
        FirstPerson {
            settings: settings,
            yaw: _0,
            pitch: _0,
            position: position,
            velocity: T::one(),
            actions: Actions::empty(),
        }
    }

    /// Computes camera.
    pub fn camera(&self, dt: T) -> Camera<T> {
        let dh = dt * self.velocity * self.settings.speed_horizontal;
        let (dx, dy, dz) = self.movement_direction();
        let (s, c) = (self.yaw.sin(), self.yaw.cos());
        let mut camera = Camera::new([
            self.position[0] + (s * dx - c * dz) * dh,
            self.position[1] + dy * dt * self.settings.speed_vertical,
            self.position[2] + (s * dz + c * dx) * dh
        ]);
        camera.set_yaw_pitch(self.yaw, self.pitch);
        camera
    }

    /// Updates the camera for an elapsed number of seconds.
    pub fn update(&mut self, dt: T) {
        let cam = self.camera(dt);
        self.position = cam.position;
    }

    /// Updates the camera for a mouse movement.
    pub fn update_mouse(&mut self, relative_dx: T, relative_dy: T) {
        let FirstPerson {
            ref mut yaw, ref mut pitch, ref settings, ..
        } = *self;

        let dx = relative_dx * settings.mouse_sensitivity_horizontal;
        let dy = relative_dy * settings.mouse_sensitivity_vertical;

        let pi: T = Radians::_180();
        let _0 = T::zero();
        let _1 = T::one();
        let _2 = _1 + _1;
        let _3 = _2 + _1;
        let _4 = _3 + _1;
        let _360 = T::from_isize(360);

        *yaw = (*yaw - dx / _360 * pi / _4) % (_2 * pi);
        *pitch = *pitch + dy / _360 * pi / _4;
        *pitch = (*pitch).min(pi / _2).max(-pi / _2);
    }

    /// Gets the direction of movement.
    pub fn movement_direction(&self) -> (T, T, T) {
        let (mut dx, mut dy, mut dz) = (T::zero(), T::zero(), T::zero());

        let set_axis = |axis: &mut T, positive, negative| {
            if self.actions.contains(positive) &&
                self.actions.contains(negative) {
                *axis = T::zero();
            } else if self.actions.contains(positive) {
                *axis = T::one();
            } else if self.actions.contains(negative) {
                *axis = -T::one();
            }
        };

        set_axis(&mut dz, Actions::MOVE_FORWARD, Actions::MOVE_BACKWARD);
        set_axis(&mut dx, Actions::STRAFE_LEFT, Actions::STRAFE_RIGHT);
        set_axis(&mut dy, Actions::FLY_UP, Actions::FLY_DOWN);

        (dx,dy,dz)
    }

    pub fn enable_actions(&mut self, actions: Actions) {
        self.actions |= actions;
    }

    pub fn disable_action(&mut self, action: Actions) {
        self.actions &= !action;
    }
}
