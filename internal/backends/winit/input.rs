use i_slint_core::{
    api::LogicalPosition,
    items::PointerEventButton,
    lengths::{LogicalPoint, LogicalPx},
};
use lyon_path::geom::euclid::Point2D;
use winit::event::DeviceId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Pointer {
    pub device_id: DeviceId,
    /// This represents the individual finger, whereas the device is the touchscreen itself, for example
    pub touch_id: Option<u64>,
    pub button: PointerEventButton,
    pub timestamp: u128,
    pub position: LogicalPoint,
}
