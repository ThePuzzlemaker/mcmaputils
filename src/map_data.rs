use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapRoot {
    pub data: MapData,
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MapData {
    pub scale: u8,
    pub dimension: String,
    #[serde(rename = "trackingPosition")]
    pub tracking_position: bool,
    #[serde(rename = "unlimitedTracking")]
    pub unlimited_tracking: bool,
    pub locked: bool,
    #[serde(rename = "xCenter")]
    pub x_center: i32,
    #[serde(rename = "zCenter")]
    pub z_center: i32,
    pub banners: Vec<Banner>,
    pub frames: Vec<Frame>,
    pub colors: Vec<i8>,
}

impl fmt::Debug for MapData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapData")
            .field("scale", &self.scale)
            .field("dimension", &self.dimension)
            .field("tracking_position", &self.tracking_position)
            .field("unlimited_tracking", &self.unlimited_tracking)
            .field("locked", &self.locked)
            .field("x_center", &self.x_center)
            .field("z_center", &self.z_center)
            .field("banners", &self.banners)
            .field("frames", &self.frames)
            .field("colors", &LargeList)
            .finish()
    }
}

struct LargeList;
impl fmt::Debug for LargeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[/* ... */]")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Banner {
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Pos")]
    pub pos: Pos,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Frame {
    #[serde(rename = "EntityId")]
    pub entity_id: u32,
    #[serde(rename = "Rotation")]
    pub rotation: u32,
    #[serde(rename = "Pos")]
    pub pos: Pos,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pos {
    #[serde(rename = "X")]
    pub x: i32,
    #[serde(rename = "Y")]
    pub y: i32,
    #[serde(rename = "Z")]
    pub z: i32,
}
