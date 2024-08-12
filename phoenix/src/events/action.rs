#[derive(Clone)]
pub enum Action {
    CameraUpdateForward,
    CameraUpdateBackward,
    CameraUpdateLeft,
    CameraUpdateRight,
    CameraFov(f32),
}