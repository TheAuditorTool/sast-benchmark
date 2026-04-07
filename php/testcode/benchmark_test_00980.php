<?php
require_once __DIR__ . '/shared.php';

class Processor043 {
    public function process(string $data): string {
        return htmlspecialchars($data, ENT_QUOTES);
    }
}

function benchmarkTest00980(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new Processor043();
    $result = call_user_func([$obj, 'process'], $req->param('data'));
    return BenchmarkResponse::ok($result);
}
