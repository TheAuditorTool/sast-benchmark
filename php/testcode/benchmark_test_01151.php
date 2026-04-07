<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01151(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $keyword = $req->param('q');
    $stmt = $pdo->prepare("SELECT id, title FROM posts WHERE title LIKE ?");
    $stmt->execute(['%' . $keyword . '%']);
    return BenchmarkResponse::json($stmt->fetchAll());
}
