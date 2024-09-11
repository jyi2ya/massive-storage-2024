mod seek_model;
mod solve;
mod utility;

/// 磁头的运动状态
#[derive(Clone, Copy, Debug)]
pub enum HeadStatus {
    /// 磁头没动
    Static = 0,
    /// 磁头正在读写
    Rw = 1,
    /// 正在倒带
    Seek = 2,
}

impl From<HeadStatus> for u32 {
    fn from(value: HeadStatus) -> Self {
        value as u32
    }
}

impl From<u32> for HeadStatus {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Static,
            1 => Self::Rw,
            2 => Self::Seek,
            _ => panic!("invalid HeadStatus variant"),
        }
    }
}

/// IO 请求的编号
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitId(u32);

impl From<u32> for UnitId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl UnitId {
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// 磁带的 Warp 序号
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapId(u32);

impl From<u32> for WrapId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl WrapId {
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// 磁头状态
#[derive(Clone, Debug)]
pub struct HeadInfo {
    /// 磁头所在的 Wrap
    pub wrap: WrapId,
    /// 磁头在 Warp 中的位置
    pub lpos: u32,
    /// 磁头的运动状态
    pub status: HeadStatus,
}

impl From<seek_model::HeadInfo> for HeadInfo {
    fn from(value: seek_model::HeadInfo) -> Self {
        let seek_model::HeadInfo { wrap, lpos, status } = value;
        Self {
            wrap: WrapId::from(wrap),
            lpos,
            status: HeadStatus::from(status),
        }
    }
}

impl From<&HeadInfo> for seek_model::HeadInfo {
    fn from(value: &HeadInfo) -> Self {
        seek_model::HeadInfo {
            wrap: value.wrap.get(),
            lpos: value.lpos,
            status: u32::from(value.status),
        }
    }
}

/// IO 请求
#[derive(Clone, Debug)]
pub struct Unit {
    /// 请求 Id
    pub id: UnitId,
    /// 磁带 Warp Id
    pub wrap: WrapId,
    /// IO 的起始位置
    pub start_lpos: u32,
    /// IO 的终了位置
    pub end_lpos: u32,
}

impl From<seek_model::IOUint> for Unit {
    fn from(value: seek_model::IOUint) -> Self {
        let seek_model::IOUint {
            id,
            wrap,
            startLpos,
            endLpos,
        } = value;
        Self {
            id: UnitId::from(id),
            wrap: WrapId::from(wrap),
            start_lpos: startLpos,
            end_lpos: endLpos,
        }
    }
}

/// # Safety
///
/// totally unsafe
///
/// 供 project_hw 调用的 C 接口。不用太关心
#[no_mangle]
pub unsafe fn solve(input: *const seek_model::InputParam, output: *mut seek_model::OutputParam) {
    let input_param = *input;
    let head_info = HeadInfo::from(input_param.headInfo);
    let io_units = Vec::from_raw_parts(
        input_param.ioVec.ioArray,
        input_param.ioVec.len as usize,
        input_param.ioVec.len as usize,
    )
    .into_iter()
    .map(Unit::from)
    .collect::<Vec<_>>()
    .leak();

    let result = solve::solve(&head_info, io_units);

    let result = result.iter().map(UnitId::get).collect::<Vec<_>>();
    std::ptr::copy(result.as_ptr(), (*output).sequence, result.len());
    (*output).len = result.len() as u32;
}

/// 虽然是 main 函数，但是并没有什么用，只是用来占位的
/// mrustc 编译没 main 的库的时候会有奇怪的问题，不太搞得定
/// 所以加了个没什么用的 main
fn main() {
    let head_info = HeadInfo {
        wrap: WrapId(8),
        lpos: 1000,
        status: HeadStatus::Static,
    };
    let io_vec = [
        Unit {
            id: UnitId(1),
            wrap: WrapId(244),
            start_lpos: 551525,
            end_lpos: 552079,
        },
        Unit {
            id: UnitId(2),
            wrap: WrapId(122),
            start_lpos: 187227,
            end_lpos: 187781,
        },
        Unit {
            id: UnitId(3),
            wrap: WrapId(155),
            start_lpos: 520928,
            end_lpos: 520374,
        },
        Unit {
            id: UnitId(4),
            wrap: WrapId(105),
            start_lpos: 462047,
            end_lpos: 461493,
        },
        Unit {
            id: UnitId(5),
            wrap: WrapId(242),
            start_lpos: 31892,
            end_lpos: 32446,
        },
        Unit {
            id: UnitId(6),
            wrap: WrapId(208),
            start_lpos: 394655,
            end_lpos: 395209,
        },
        Unit {
            id: UnitId(7),
            wrap: WrapId(255),
            start_lpos: 212911,
            end_lpos: 212357,
        },
        Unit {
            id: UnitId(8),
            wrap: WrapId(172),
            start_lpos: 520584,
            end_lpos: 521138,
        },
        Unit {
            id: UnitId(9),
            wrap: WrapId(208),
            start_lpos: 414564,
            end_lpos: 415118,
        },
        Unit {
            id: UnitId(10),
            wrap: WrapId(153),
            start_lpos: 682905,
            end_lpos: 682351,
        },
        Unit {
            id: UnitId(11),
            wrap: WrapId(51),
            start_lpos: 269102,
            end_lpos: 268548,
        },
        Unit {
            id: UnitId(12),
            wrap: WrapId(219),
            start_lpos: 485628,
            end_lpos: 485074,
        },
        Unit {
            id: UnitId(13),
            wrap: WrapId(246),
            start_lpos: 393485,
            end_lpos: 394039,
        },
        Unit {
            id: UnitId(14),
            wrap: WrapId(267),
            start_lpos: 997,
            end_lpos: 443,
        },
        Unit {
            id: UnitId(15),
            wrap: WrapId(88),
            start_lpos: 185046,
            end_lpos: 185600,
        },
    ];
    let result = solve::solve(&head_info, &io_vec[..]);
    println!("{:?}", result);

    // magic, do not touch
    println!("{:?}", solve as *const u8);
}
