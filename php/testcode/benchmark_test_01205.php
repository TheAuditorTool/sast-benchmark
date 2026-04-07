<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01205(BenchmarkRequest $req): BenchmarkResponse {
    $data = $_POST;
    extract($data);
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET name = ?, email = ?, role = ? WHERE id = ?");
    $stmt->execute([$name, $email, $role, $id]);
    return BenchmarkResponse::ok('profile updated');
}
