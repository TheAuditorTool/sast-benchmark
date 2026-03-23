<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_no_token
function csrf001(BenchmarkRequest $req): BenchmarkResponse {
    $email = $req->post('email');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $req->post('user_id')]); // vuln-code-snippet vuln-line php_csrf_no_token
    return BenchmarkResponse::ok('Email updated');
}
// vuln-code-snippet end php_csrf_no_token
