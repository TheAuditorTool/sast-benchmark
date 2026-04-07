<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01184(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->post('csrf_token');
    $session = $req->cookie('session_token');
    if (!hash_equals($session, $token)) {
        return BenchmarkResponse::error('forbidden');
    }
    $pdo = getDbConnection();
    $email = $req->post('email');
    $uid = $req->post('user_id');
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $uid]);
    return BenchmarkResponse::ok('email updated');
}
