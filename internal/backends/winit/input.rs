use i_slint_core::{
    api::LogicalPosition,
    items::PointerEventButton,
    lengths::{LogicalPoint, LogicalPx},
};
use lyon_path::geom::euclid::Point2D;

pub(crate) struct Pointer {
    pub id: u64,
    pub button: PointerEventButton,
    pub timestamp: u128,
    pub position: LogicalPoint,
}
