use crate::{seek_model, HeadInfo, HeadStatus, Unit, UnitId, WrapId};
use std::collections::HashMap;

/// 获取磁带寻道的耗时
pub fn get_seek_time(start: &HeadInfo, target: &HeadInfo) -> u32 {
    let start_head = seek_model::HeadInfo::from(start);
    let target_head = seek_model::HeadInfo::from(target);
    unsafe {
        let result = seek_model::SeekTimeCalculate(
            &start_head as *const seek_model::HeadInfo,
            &target_head as *const seek_model::HeadInfo,
        );
        result
    }
}

/// 获取电机磨损次数
pub fn get_motor_wear_times(start: &HeadInfo, target: &HeadInfo) -> u32 {
    let start_head = seek_model::HeadInfo::from(start);
    let target_head = seek_model::HeadInfo::from(target);
    unsafe {
        seek_model::MotorWearTimes(
            &start_head as *const seek_model::HeadInfo,
            &target_head as *const seek_model::HeadInfo,
        )
    }
}

/// 获取磁带读数据的耗时
pub fn get_read_time(start: u32, target: u32) -> u32 {
    let start = start as i32;
    let target = target as i32;
    unsafe { seek_model::ReadTimeCalculate(i32::abs(start - target) as u32) }
}

/// 传入当前磁头位置和 IO 请求，返回完成 IO 请求后的磁头位置，和完成 IO 请求的耗时
pub fn apply_io_unit(head: &HeadInfo, io: &Unit) -> (HeadInfo, u32) {
    let read_start = HeadInfo {
        wrap: io.wrap,
        lpos: io.start_lpos,
        status: HeadStatus::Rw,
    };
    let seek_time = get_seek_time(head, &read_start);
    let read_time = get_read_time(io.start_lpos, io.end_lpos);
    let head_end_pos = HeadInfo {
        wrap: io.wrap,
        lpos: io.end_lpos,
        status: HeadStatus::Rw,
    };
    (head_end_pos, seek_time + read_time)
}

/// 传入一个 IO 请求排序方案，和初始磁头位置
/// 返回完成该方案的总耗时
pub fn get_case_score(head: &HeadInfo, plan: &[Unit]) -> u32 {
    let seek_starts = std::iter::once(head.to_owned())
        .chain(plan.iter().map(|unit| HeadInfo {
            wrap: unit.wrap,
            lpos: unit.end_lpos,
            status: HeadStatus::Seek,
        }))
        .collect::<Vec<_>>();
    let seek_ends = plan
        .iter()
        .map(|unit| HeadInfo {
            wrap: unit.wrap,
            lpos: unit.start_lpos,
            status: HeadStatus::Rw,
        })
        .collect::<Vec<_>>();

    let seek_cost = (0..usize::min(seek_starts.len(), seek_ends.len()))
        .into_iter()
        .map(|idx| get_seek_time(&seek_starts[idx], &seek_ends[idx]))
        .sum::<u32>();
    let read_cost = plan
        .iter()
        .map(|unit| get_read_time(unit.start_lpos, unit.end_lpos))
        .sum::<u32>();
    seek_cost + read_cost
}

#[cfg(test)]
mod test {
    use crate::{HeadInfo, HeadStatus, WrapId};

    #[test]
    fn get_seek_time() {
        let result = super::get_seek_time(
            &HeadInfo {
                wrap: WrapId::from(1),
                lpos: 1,
                status: HeadStatus::Static,
            },
            &HeadInfo {
                wrap: WrapId::from(1),
                lpos: 6,
                status: HeadStatus::Static,
            },
        );
        assert_eq!(result, 10705);
    }

    #[test]
    fn get_motor_wear_times() {
        let result = super::get_motor_wear_times(
            &HeadInfo {
                wrap: WrapId::from(4),
                lpos: 15,
                status: HeadStatus::Seek,
            },
            &HeadInfo {
                wrap: WrapId::from(3),
                lpos: 20,
                status: HeadStatus::Rw,
            },
        );
        assert_eq!(result, 2);
    }
}
