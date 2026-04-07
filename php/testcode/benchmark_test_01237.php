<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01237(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    extract($_GET);
    if ($isAdmin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
