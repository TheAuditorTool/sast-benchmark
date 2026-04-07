<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01223(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $input = $req->param('token');
    $stmt = $pdo->prepare("SELECT value FROM tokens WHERE id = 1");
    $stmt->execute();
    $row = $stmt->fetch();
    if (hash_equals($row['value'], $input)) {
        return BenchmarkResponse::ok('access granted');
    }
    return BenchmarkResponse::error('denied');
}
