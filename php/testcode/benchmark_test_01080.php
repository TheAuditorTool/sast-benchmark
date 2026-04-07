<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01080(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = json_decode($req->bodyStr(), true);
    $sets = [];
    $values = [];
    foreach ($data as $col => $val) {
        $sets[] = "$col = ?";
        $values[] = $val;
    }
    $values[] = $req->param('id');
    $pdo->prepare("UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?")->execute($values);
    return BenchmarkResponse::ok('User updated');
}
