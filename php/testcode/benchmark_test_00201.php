<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00201(BenchmarkRequest $req): BenchmarkResponse {
    $amount = $req->param('amount');
    if (!is_numeric($amount)) {
        return BenchmarkResponse::badRequest('Amount must be numeric');
    }
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        ->execute([$amount, $req->param('account_id')]);
    return BenchmarkResponse::ok('Transaction processed');
}
