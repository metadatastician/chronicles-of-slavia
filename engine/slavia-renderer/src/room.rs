//! Mapping between a room's local beat units and its L3 horizontal landmarks.
//! Kept bidirectional so Continue spawns at the saved location, not the entrance.

#[derive(Debug)]
pub struct PathView {
    landmarks: Vec<f32>,
}

impl PathView {
    pub fn new(landmarks: Vec<f32>) -> Result<Self, &'static str> {
        if landmarks.is_empty() || landmarks.iter().any(|x| !x.is_finite())
            || landmarks.windows(2).any(|p| p[0] >= p[1])
        {
            return Err("landmarks must be nonempty, finite and strictly increasing");
        }
        Ok(Self { landmarks })
    }

    pub fn beat_for_x(&self, x: f32) -> f32 {
        for (i, pair) in self.landmarks.windows(2).enumerate() {
            if x <= pair[1] {
                return i as f32 + ((x - pair[0]) / (pair[1] - pair[0])).clamp(0.0, 1.0);
            }
        }
        (self.landmarks.len() - 1) as f32
    }

    pub fn x_for_beat(&self, beat: f32) -> f32 {
        let beat = beat.clamp(0.0, (self.landmarks.len() - 1) as f32);
        let i = beat as usize;
        let next = (i + 1).min(self.landmarks.len() - 1);
        self.landmarks[i] + (self.landmarks[next] - self.landmarks[i]) * beat.fract()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trips_fractional_positions_and_clamps_edges() {
        let path = PathView::new(vec![-520.0, -300.0, 150.0, 2200.0]).unwrap();
        for beat in [0.0, 0.25, 1.0, 1.75, 2.4, 3.0] {
            assert!((path.beat_for_x(path.x_for_beat(beat)) - beat).abs() < 0.00001);
        }
        assert_eq!(path.beat_for_x(-1000.0), 0.0);
        assert_eq!(path.beat_for_x(3000.0), 3.0);
        assert_eq!(path.x_for_beat(-1.0), -520.0);
        assert_eq!(path.x_for_beat(4.0), 2200.0);
    }
    #[test]
    fn validates_and_supports_single_beat_rooms() {
        for points in [vec![], vec![f32::NAN], vec![1.0, 1.0], vec![2.0, 1.0]] {
            assert!(PathView::new(points).is_err());
        }
        let path = PathView::new(vec![42.0]).unwrap();
        assert_eq!(path.beat_for_x(500.0), 0.0);
        assert_eq!(path.x_for_beat(0.0), 42.0);
    }
}
