<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_token_verified
function csrf002(BenchmarkRequest $req): BenchmarkResponse {
    $sessionToken = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($sessionToken, $req->post('csrf_token'))) { // vuln-code-snippet safe-line php_csrf_token_verified
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    $email = $req->post('email');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $req->post('user_id')]);
    return BenchmarkResponse::ok('Email updated');
}
// vuln-code-snippet end php_csrf_token_verified
