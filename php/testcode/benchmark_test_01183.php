<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01183(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $amount = $req->post('amount');
    $to = $req->post('recipient');
    $from = $req->cookie('user_id');
    $stmt = $pdo->prepare("INSERT INTO transfers (from_id, to_id, amount) VALUES (?, ?, ?)");
    $stmt->execute([$from, $to, $amount]);
    return BenchmarkResponse::ok('transferred');
}
