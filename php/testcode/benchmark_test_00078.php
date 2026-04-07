<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00078(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $token = md5($userId . 'salt');
    setcookie('remember', $token, time() + 86400 * 30, '/', '', true, true);
    return BenchmarkResponse::ok('remember me set');
}
