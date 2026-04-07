<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00315(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    parse_str($req->bodyStr());
    if ($is_admin) {
        return BenchmarkResponse::ok('admin access');
    }
    return BenchmarkResponse::ok('user access');
}
