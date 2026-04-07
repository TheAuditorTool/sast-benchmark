<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00090(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    extract($req->postData);
    if ($is_admin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
