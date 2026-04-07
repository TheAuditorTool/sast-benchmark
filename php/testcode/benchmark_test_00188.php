<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00188(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $opts = $req->param('opts');
    parse_str($opts);
    if ($is_admin) {
        return BenchmarkResponse::ok('Admin access granted');
    }
    return BenchmarkResponse::ok('Regular user');
}
