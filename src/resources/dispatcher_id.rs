#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DispatcherId {
    MainMenu,
    LoadIngame,
    Ingame,
    Paused,
    GameOver,
}
