<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_array_walk_method
class DataProcessor029 {
    public function normalize(string $value, mixed $key): void {}
    public function sanitize(string $value, mixed $key): void {}
}

function codeinj029(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->post('method');
    $obj = new DataProcessor029();
    $arr = ['foo', 'bar', 'baz'];
    array_walk($arr, [$obj, $method]); // vuln-code-snippet vuln-line php_codeinj_array_walk_method
    return BenchmarkResponse::ok(implode(',', $arr));
}
// vuln-code-snippet end php_codeinj_array_walk_method
