<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00146(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('user_id');
    $id = intval($input);
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $user = $stmt->fetch(\PDO::FETCH_ASSOC);
    return BenchmarkResponse::json($user ?: ['error' => 'not found']);
}
