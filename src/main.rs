mod seek_model;

#[no_mangle]
pub unsafe fn solve(input: *const seek_model::InputParam, output: *mut seek_model::OutputParam) {
    dbg!(*input);
    dbg!(*output);
}

fn main() {
    let start = seek_model::HeadInfo {
        wrap: 10,
        lpos: 1000,
        status: seek_model::HEAD_STATUS_HEAD_STATIC,
    };
    let end = seek_model::HeadInfo {
        wrap: 15,
        lpos: 3000,
        status: seek_model::HEAD_STATUS_HEAD_RW,
    };
    let result = unsafe { seek_model::SeekTimeCalculate(&start, &end) };
    println!("{}", result);
    let result = unsafe {
        solve(
            0 as *const seek_model::InputParam,
            0 as *mut seek_model::OutputParam,
        )
    };
}
