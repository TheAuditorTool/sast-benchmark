<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00247(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    $input = $req->param('flag');
    $$input = true;
    if ($isAdmin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
