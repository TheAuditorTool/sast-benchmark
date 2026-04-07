<?php
require_once __DIR__ . '/shared.php';

class DataProcessor {
    public function process(string $data): string { return strtoupper($data); }
}

function benchmarkTest00768(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $obj = new DataProcessor();
    $result = call_user_func([$obj, 'process'], $data);
    return BenchmarkResponse::ok($result);
}
