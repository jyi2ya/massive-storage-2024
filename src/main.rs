mod seek_model;
mod solve;

#[derive(Clone, Copy, Debug)]
pub enum HeadStatus {
    Static = 0,
    Rw = 1,
    Butt = 2,
}

impl From<u32> for HeadStatus {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Static,
            1 => Self::Rw,
            2 => Self::Butt,
            _ => panic!("invalid HeadStatus variant"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct HeadInfo {
    pub wrap: WrapId,
    pub lpos: u32,
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

#[derive(Clone, Debug)]
pub struct Unit {
    pub id: UnitId,
    pub wrap: WrapId,
    pub start_lpos: u32,
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
/// trust me
#[no_mangle]
pub unsafe fn solve(input: *const seek_model::InputParam, output: *mut seek_model::OutputParam) {
    let input_param = *input;
    let head_info = HeadInfo::from(input_param.headInfo);
    let io_vec = Vec::from_raw_parts(
        input_param.ioVec.ioArray,
        input_param.ioVec.len as usize,
        input_param.ioVec.len as usize,
    )
    .into_iter()
    .map(Unit::from)
    .collect::<Vec<_>>()
    .leak();
    let result = solve::solve(head_info, io_vec)
        .iter()
        .map(UnitId::get)
        .collect::<Vec<_>>();
    std::ptr::copy(result.as_ptr(), (*output).sequence, result.len());
    (*output).len = result.len() as u32;
}

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
    let result = solve::solve(head_info, &io_vec[..]);
    println!("{:?}", result);

    // magic, do not touch
    std::hint::black_box(solve);
}
