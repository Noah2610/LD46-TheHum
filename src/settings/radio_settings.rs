// resources/settings/radio.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct RadioSettings {
    pub size:      Size,
    pub animation: Option<Animation>,
}
