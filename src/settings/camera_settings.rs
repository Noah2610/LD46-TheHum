// resources/settings/camera.ron

use crate::components::prelude::Size;

#[derive(Clone, Deserialize)]
pub struct CameraSettings {
    pub z:    f32,
    pub size: Size,
}
