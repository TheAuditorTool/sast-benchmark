<?php
require_once __DIR__ . '/shared.php';

class EvalGadget032 {
    public string $code = '';
    public function __wakeup(): void {
        eval($this->code);
    }
}

function benchmarkTest00403(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data);
    return BenchmarkResponse::ok('deserialized');
}
