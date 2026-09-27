//! Authored L3 staging for A1, validated against the SGS room it draws.
use serde::Deserialize;
use slavia_core::room::Room;
use slavia_renderer::{camera::Bounds, room::PathView};

#[derive(Deserialize)]
struct ViewData {
    room: String,
    world_min: [f32; 2],
    world_max: [f32; 2],
    beat_x: Vec<f32>,
    camera_response: f32,
}

pub struct RoomView {
    pub bounds: Bounds,
    pub path: PathView,
    pub camera_response: f32,
}

impl RoomView {
    pub fn border_path(room: &Room) -> Self {
        Self::parse(include_str!("../data/border-path.view.toml"), room)
            .expect("bundled A1 staging matches its SGS room")
    }

    fn parse(source: &str, room: &Room) -> Result<Self, String> {
        let data: ViewData = toml::from_str(source).map_err(|e| e.to_string())?;
        if data.room != room.id || data.beat_x.len() != room.beats.len() {
            return Err("view must match the room id and local beat count".into());
        }
        let bounds = Bounds::new(data.world_min, data.world_max)?;
        if !data.camera_response.is_finite() || data.camera_response <= 0.0
            || data.beat_x.iter().any(|x| *x < data.world_min[0] || *x > data.world_max[0])
        {
            return Err("invalid camera response or out-of-bounds landmark".into());
        }
        Ok(Self {
            bounds,
            path: PathView::new(data.beat_x)?,
            camera_response: data.camera_response,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a1_staging_preserves_the_existing_landmarks() {
        let spec = slavia_core::zone_a();
        let view = RoomView::border_path(&spec.rooms[0]);
        assert_eq!(view.path.x_for_beat(1.0), -300.0);
        assert_eq!(view.path.x_for_beat(3.0), 0.0);
        assert_eq!(view.bounds.clamp_point([1000.0, 0.0]), [620.0, 0.0]);
    }
    #[test]
    fn refuses_a_view_for_the_wrong_room() {
        let mut room = slavia_core::zone_a().rooms.remove(0);
        room.id = "another-room".into();
        assert!(RoomView::parse(include_str!("../data/border-path.view.toml"), &room).is_err());
    }
}
