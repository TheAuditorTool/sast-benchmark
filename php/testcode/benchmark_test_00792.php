<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00792(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $token = $userId . ':' . date('YmdHis');
    setcookie('remember', $token, time() + 86400 * 30, '/', '', true, true);
    return BenchmarkResponse::ok('remember me set');
}
