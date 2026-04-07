<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01243(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->post('user_id');
    setcookie('user_id', $userId, time() + 86400);
    return BenchmarkResponse::ok('logged in');
}
