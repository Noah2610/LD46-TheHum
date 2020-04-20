// resources/settings/general.ron

#[derive(Clone, Deserialize)]
pub struct GeneralSettings {
    pub player_animation_update_velocity_padding: f32,
    pub load_ingame_state_duration_ms:            u64,
}
