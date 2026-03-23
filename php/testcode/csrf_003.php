<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_get_state_change
function csrf003(BenchmarkRequest $req): BenchmarkResponse {
    $postId = $req->param('id');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM posts WHERE id = ?");
    $stmt->execute([$postId]); // vuln-code-snippet vuln-line php_csrf_get_state_change
    return BenchmarkResponse::ok('Post deleted');
}
// vuln-code-snippet end php_csrf_get_state_change
