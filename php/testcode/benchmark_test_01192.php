<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01192(BenchmarkRequest $req): BenchmarkResponse {
    $adminHash = getenv('ADMIN_PASSWORD_HASH');
    $provided = $req->post('password');
    if (password_verify($provided, $adminHash)) {
        return BenchmarkResponse::ok('access granted');
    }
    return BenchmarkResponse::error('denied');
}
