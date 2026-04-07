use std::num::Wrapping;

static mut COUNTER: u64 = 0;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");
    let token = unsafe {
        COUNTER = (Wrapping(COUNTER) + Wrapping(1103515245)).0;
        COUNTER
    };
    super::shared::BenchmarkResponse::ok(&format!("Reset token: {:x}", token))
}
