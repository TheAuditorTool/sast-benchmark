<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00586(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $pdo->prepare("SELECT name, email, role FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $existing = $stmt->fetch(\PDO::FETCH_ASSOC);
    $updated = array_replace($existing, $req->postData);
    $pdo->prepare("UPDATE users SET name = ?, email = ?, role = ? WHERE id = ?")
        ->execute([$updated['name'], $updated['email'], $updated['role'], $id]);
    return BenchmarkResponse::ok('User patched');
}
