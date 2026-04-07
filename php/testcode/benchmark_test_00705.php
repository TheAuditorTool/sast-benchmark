<?php
require_once __DIR__ . '/shared.php';

class DataProcessor029 {
    public function normalize(string $value, mixed $key): void {}
    public function sanitize(string $value, mixed $key): void {}
}

function benchmarkTest00705(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->post('method');
    $obj = new DataProcessor029();
    $arr = ['foo', 'bar', 'baz'];
    array_walk($arr, [$obj, $method]);
    return BenchmarkResponse::ok(implode(',', $arr));
}
