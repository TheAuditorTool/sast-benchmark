<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_token_localstorage
function csrf025(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = bin2hex(random_bytes(32));
    $_SESSION['csrf_token'] = $token;
    $html = '<script>localStorage.setItem("csrf_token", "' . $token . '");</script>'; // vuln-code-snippet vuln-line php_csrf_token_localstorage
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_csrf_token_localstorage
