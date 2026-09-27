//! Headless camera tracking math shared by renderer skins. World units here
//! belong to L3; none of these coordinates enter slavia-core.

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    min: [f32; 2],
    max: [f32; 2],
}

impl Bounds {
    pub fn new(min: [f32; 2], max: [f32; 2]) -> Result<Self, &'static str> {
        if (0..2).any(|i| !min[i].is_finite() || !max[i].is_finite() || min[i] >= max[i]) {
            return Err("room bounds must be finite and increasing");
        }
        Ok(Self { min, max })
    }

    pub fn clamp_point(&self, mut point: [f32; 2]) -> [f32; 2] {
        for (i, value) in point.iter_mut().enumerate() {
            *value = (*value).clamp(self.min[i], self.max[i]);
        }
        point
    }

    fn camera_center(&self, mut point: [f32; 2], half_view: [f32; 2]) -> [f32; 2] {
        for (i, value) in point.iter_mut().enumerate() {
            let low = self.min[i] + half_view[i];
            let high = self.max[i] - half_view[i];
            *value = if low <= high {
                (*value).clamp(low, high)
            } else {
                // Never invert clamp bounds when a window exceeds the room.
                (self.min[i] + self.max[i]) * 0.5
            };
        }
        point
    }
}

#[derive(Debug, Default)]
pub struct CameraFollow {
    center: Option<[f32; 2]>,
}

impl CameraFollow {
    /// Snap on first use (new game / restored session / room entry), then use
    /// exponential smoothing. Partitioning the same elapsed time gives the
    /// same stationary-target result, unlike a fixed per-frame lerp factor.
    pub fn update(
        &mut self,
        target: [f32; 2],
        bounds: Bounds,
        half_view: [f32; 2],
        response: f32,
        dt: f32,
    ) -> Option<[f32; 2]> {
        if target.iter().any(|v| !v.is_finite())
            || half_view.iter().any(|v| !v.is_finite() || *v < 0.0)
            || !response.is_finite() || response <= 0.0
            || !dt.is_finite() || dt < 0.0
        {
            return self.center;
        }
        let target = bounds.camera_center(target, half_view);
        let mut center = self.center.unwrap_or(target);
        let alpha = 1.0 - (-response * dt).exp();
        for i in 0..2 {
            center[i] += (target[i] - center[i]) * alpha;
        }
        // Re-clamp after smoothing so a resize cannot leave the view outside.
        center = bounds.camera_center(center, half_view);
        self.center = Some(center);
        self.center
    }

    pub fn reset(&mut self) {
        self.center = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wide() -> Bounds {
        Bounds::new([-2000.0, -320.0], [2000.0, 320.0]).unwrap()
    }
    #[test]
    fn follows_beyond_the_old_screen_without_overshoot() {
        let mut camera = CameraFollow::default();
        assert_eq!(camera.update([0.0, 0.0], wide(), [590.0, 320.0], 8.0, 0.0), Some([0.0, 0.0]));
        let pos = camera.update([1500.0, 100.0], wide(), [590.0, 320.0], 8.0, 1.0).unwrap();
        assert!(pos[0] > 620.0 && pos[0] <= 1410.0);
        assert_eq!(pos[1], 0.0);
    }
    #[test]
    fn smaller_rooms_center_and_resize_clamps_immediately() {
        let mut camera = CameraFollow::default();
        camera.update([1500.0, 0.0], wide(), [590.0, 320.0], 8.0, 1.0);
        assert_eq!(camera.update([1500.0, 0.0], wide(), [2500.0, 800.0], 8.0, 0.0), Some([0.0, 0.0]));
    }
    #[test]
    fn smoothing_is_time_based_and_reset_snaps() {
        let mut a = CameraFollow::default();
        let mut b = CameraFollow::default();
        for c in [&mut a, &mut b] {
            c.update([0.0, 0.0], wide(), [0.0; 2], 8.0, 0.0);
        }
        let one = a.update([1000.0, 0.0], wide(), [0.0; 2], 8.0, 0.5).unwrap();
        b.update([1000.0, 0.0], wide(), [0.0; 2], 8.0, 0.25);
        let two = b.update([1000.0, 0.0], wide(), [0.0; 2], 8.0, 0.25).unwrap();
        assert!((one[0] - two[0]).abs() < 0.001);
        b.reset();
        assert_eq!(b.update([-1000.0, 0.0], wide(), [0.0; 2], 8.0, 0.0), Some([-1000.0, 0.0]));
    }
    #[test]
    fn rejects_invalid_geometry_and_time() {
        assert!(Bounds::new([1.0, 0.0], [0.0, 1.0]).is_err());
        assert!(Bounds::new([f32::NAN, 0.0], [1.0, 1.0]).is_err());
        let mut camera = CameraFollow::default();
        assert_eq!(camera.update([f32::NAN, 0.0], wide(), [0.0; 2], 8.0, 1.0), None);
        assert_eq!(camera.update([0.0; 2], wide(), [0.0; 2], 8.0, -1.0), None);
    }
}
