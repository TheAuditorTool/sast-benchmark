<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01185(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->post('_token');
    $stored = $req->cookie('xsrf_token');
    if (!hash_equals($stored, $token)) {
        return BenchmarkResponse::error('invalid request');
    }
    $pdo = getDbConnection();
    $uid = $req->post('user_id');
    $stmt = $pdo->prepare("DELETE FROM accounts WHERE id = ?");
    $stmt->execute([$uid]);
    return BenchmarkResponse::ok('account removed');
}
