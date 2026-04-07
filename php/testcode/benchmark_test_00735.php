<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00735(BenchmarkRequest $req): BenchmarkResponse {
    $postId = $req->post('post_id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM wp_posts WHERE ID = ?");
    $stmt->execute([$postId]);
    return BenchmarkResponse::ok('Post deleted');
}
