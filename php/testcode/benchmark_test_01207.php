<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01207(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $email = $req->post('email');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET name = ?, email = ? WHERE id = ?");
    $stmt->execute([$name, $email, $req->post('id')]);
    return BenchmarkResponse::ok('profile updated');
}
