use std::mem::MaybeUninit;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _tag = req.param("tag");

    let buf: [u8; 64] = unsafe {
        MaybeUninit::<[u8; 64]>::uninit().assume_init()
    };

    let checksum: u64 = buf.iter().map(|&b| b as u64).sum();

    super::shared::BenchmarkResponse::ok(&format!("Checksum: {}", checksum))
}
