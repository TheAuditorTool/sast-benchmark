<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00092(BenchmarkRequest $req): BenchmarkResponse {
    $postId = $req->param('id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM posts WHERE id = ?");
    $stmt->execute([$postId]);
    return BenchmarkResponse::ok('Post deleted');
}
