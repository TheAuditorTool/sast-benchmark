<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00921(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->post('type');
    $obj = new $type();
    return BenchmarkResponse::ok('created');
}
