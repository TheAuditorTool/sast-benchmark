<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_wp_nonce
function csrf006(BenchmarkRequest $req): BenchmarkResponse {
    if (!wp_verify_nonce($req->post('_wpnonce'), 'delete_post')) { // vuln-code-snippet safe-line php_csrf_wp_nonce
        return BenchmarkResponse::badRequest('Invalid nonce');
    }
    $postId = $req->post('post_id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM wp_posts WHERE ID = ?");
    $stmt->execute([$postId]);
    return BenchmarkResponse::ok('Post deleted');
}
// vuln-code-snippet end php_csrf_wp_nonce
