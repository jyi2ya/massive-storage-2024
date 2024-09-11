use crate::{utility, HeadInfo, Unit};

use super::Solve;

extern "C" {
    pub fn rand() -> u32;
}

fn c_rand(rand_max: usize) -> usize {
    unsafe { rand() as usize % rand_max }
}

fn probability_take(prob: f64) -> bool {
    let value = unsafe { rand() };
    let normalized = f64::from(value) / 2147483647.0;
    normalized < prob
}
pub struct Solver;

impl Solve for Solver {
    fn name(&self) -> &'static str {
        "simulated annealing"
    }

    fn solve(&self, head_info: &HeadInfo, io_units: &[Unit]) -> Vec<Unit> {
        let t_0 = 100000000000.0; // 初始温度
        let d = 0.9998; // 降温系数
        let t_k = 0.0001; // 终了温度

        let mut plan = io_units.to_owned();
        let mut best_score = utility::get_case_score(head_info, plan.as_slice());
        let mut t = t_0;
        while t >= t_k {
            let boundary_a = c_rand(plan.len());
            let boundary_b = c_rand(plan.len());

            let (boundary_a, boundary_b) = if boundary_a < boundary_b {
                (boundary_a, boundary_b)
            } else {
                (boundary_b, boundary_a)
            };

            plan[boundary_a..=boundary_b].reverse();

            let current_score = utility::get_case_score(head_info, plan.as_slice());

            if probability_take(((best_score as f64 - current_score as f64) / t).exp()) {
                best_score = current_score;
            } else {
                plan[boundary_a..=boundary_b].reverse();
            }

            t *= d;
        }

        plan
    }
}
