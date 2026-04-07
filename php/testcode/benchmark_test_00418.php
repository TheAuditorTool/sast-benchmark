<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00418(BenchmarkRequest $req): BenchmarkResponse {
    if (!wp_verify_nonce($req->post('_wpnonce'), 'delete_post')) {
        return BenchmarkResponse::badRequest('Invalid nonce');
    }
    $postId = $req->post('post_id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM wp_posts WHERE ID = ?");
    $stmt->execute([$postId]);
    return BenchmarkResponse::ok('Post deleted');
}
