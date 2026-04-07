<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_token_in_url
function csrf016(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = $_SESSION['csrf_token'] ?? bin2hex(random_bytes(16));
    $_SESSION['csrf_token'] = $token;
    $actionUrl = '/transfer?amount=100&to=attacker&token=' . $token; // vuln-code-snippet vuln-line php_csrf_token_in_url
    return BenchmarkResponse::ok($actionUrl);
}
// vuln-code-snippet end php_csrf_token_in_url
