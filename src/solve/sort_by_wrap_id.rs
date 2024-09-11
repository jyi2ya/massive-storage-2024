use crate::{HeadInfo, Unit};

use super::Solve;

pub struct Solver;

impl Solve for Solver {
    fn name(&self) -> &'static str {
        "sort by wrap id"
    }

    fn solve(&self, _head_info: &HeadInfo, io_units: &[Unit]) -> Vec<Unit> {
        let mut plan = io_units.to_owned();
        plan.sort_by_key(|unit| unit.wrap);
        plan
    }
}
