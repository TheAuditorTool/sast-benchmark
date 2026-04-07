<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01186(BenchmarkRequest $req): BenchmarkResponse {
    $formToken = $req->post('form_token');
    $cookieToken = $req->cookie('form_token');
    if (!hash_equals($cookieToken, $formToken)) {
        return BenchmarkResponse::error('request validation failed');
    }
    $pdo = getDbConnection();
    $amount = $req->post('amount');
    $to = $req->post('recipient');
    $from = $req->cookie('user_id');
    $stmt = $pdo->prepare("INSERT INTO transfers (from_id, to_id, amount) VALUES (?, ?, ?)");
    $stmt->execute([$from, $to, $amount]);
    return BenchmarkResponse::ok('transferred');
}
