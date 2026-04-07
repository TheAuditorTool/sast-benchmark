<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00445(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $auth_token = null;
    extract($req->queryParams);
    if ($is_admin) {
        return BenchmarkResponse::ok('admin dashboard');
    }
    return BenchmarkResponse::error('forbidden', 403);
}
