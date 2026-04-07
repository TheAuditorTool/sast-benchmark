<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00413(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'guest';
    $data = $req->postData;
    extract($data, EXTR_SKIP);
    return BenchmarkResponse::ok("role: $role");
}
