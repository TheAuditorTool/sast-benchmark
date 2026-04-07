<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00466(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $name = $req->post('name');
    $email = $req->post('email');
    $stmt = $pdo->prepare("UPDATE users SET name = ?, email = ? WHERE id = ?");
    $stmt->execute([$name, $email, $req->param('id')]);
    return BenchmarkResponse::json(['updated' => true]);
}
