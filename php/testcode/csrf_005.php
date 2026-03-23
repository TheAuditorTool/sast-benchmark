<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_wp_no_nonce
function csrf005(BenchmarkRequest $req): BenchmarkResponse {
    $postId = $req->post('post_id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM wp_posts WHERE ID = ?");
    $stmt->execute([$postId]); // vuln-code-snippet vuln-line php_csrf_wp_no_nonce
    return BenchmarkResponse::ok('Post deleted');
}
// vuln-code-snippet end php_csrf_wp_no_nonce
