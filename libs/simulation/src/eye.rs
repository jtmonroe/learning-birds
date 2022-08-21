use std::f32::consts::{FRAC_PI_4, PI};

use crate::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

// TODO: 100% Test Coverage
impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods{
            let vec: na::Vector2<f32> = food.position - position;
            let distance = vec.norm();
            
            if distance >= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(
                &na::Vector2::x(), 
                &vec
            ).angle() - rotation.angle(); 

            let angle = na::wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let cell = (angle + self.fov_angle / 2.0) / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - distance) / self.fov_range;
            cells[cell] = energy;
        };
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self {
            fov_range: FOV_RANGE,
            fov_angle: FOV_ANGLE,
            cells: CELLS,
        }
    }
}
