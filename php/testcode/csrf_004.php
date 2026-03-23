<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_samesite_strict
function csrf004(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['samesite' => 'Strict', 'secure' => true, 'httponly' => true]);
    session_start();
    $sessionToken = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($sessionToken, $req->post('csrf_token'))) { // vuln-code-snippet safe-line php_csrf_samesite_strict
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("DELETE FROM posts WHERE id = ?");
    $stmt->execute([$req->post('post_id')]);
    return BenchmarkResponse::ok('Post deleted');
}
// vuln-code-snippet end php_csrf_samesite_strict
