<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00909(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $pdo->prepare('SELECT redirect_url FROM redirects WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    header('Location: ' . $row['redirect_url']);
    return BenchmarkResponse::ok('');
}
