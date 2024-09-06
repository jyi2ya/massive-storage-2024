use crate::{HeadInfo, Unit, UnitId};

pub fn solve(_head_info: HeadInfo, io_vec: &[Unit]) -> Vec<UnitId> {
    io_vec.iter().map(|unit| unit.id).collect()
}
