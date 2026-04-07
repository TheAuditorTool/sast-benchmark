<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00673(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $data = json_decode($req->bodyStr(), true);
    extract($data);
    if ($is_admin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
