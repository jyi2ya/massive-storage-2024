mod annealing;
mod sort_by_end_pos;
mod sort_by_start_pos;
mod sort_by_wrap_id;

use crate::{utility, HeadInfo, Unit, UnitId};

pub trait Solve {
    fn name(&self) -> &'static str;
    fn solve(&self, head_info: &HeadInfo, io_units: &[Unit]) -> Vec<Unit>;
}

/// 算法核心
pub fn solve(head_info: &HeadInfo, io_units: &[Unit]) -> Vec<UnitId> {
    let strategy: Vec<Box<dyn Solve>> = vec![
        Box::new(sort_by_start_pos::Solver),
        Box::new(sort_by_end_pos::Solver),
        Box::new(sort_by_wrap_id::Solver),
        Box::new(annealing::Solver),
    ];
    let mut results = strategy
        .into_iter()
        .map(|solver| {
            let plan = solver.solve(head_info, io_units);
            let score = utility::get_case_score(head_info, plan.as_slice());
            (solver, plan)
        })
        .collect::<Vec<_>>();
    results
        .sort_by_cached_key(|(_solver, plan)| utility::get_case_score(head_info, plan.as_slice()));
    for (solver, plan) in &results {
        println!(
            "{}: {}",
            solver.name(),
            utility::get_case_score(head_info, plan.as_slice())
        );
    }
    results[0].1.iter().map(|unit| unit.id).collect()
}
