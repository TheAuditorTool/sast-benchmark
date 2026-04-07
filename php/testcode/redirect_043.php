<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_encrypted_session_target
function redirect043(BenchmarkRequest $req): BenchmarkResponse {
    $target = $_SESSION['redirect_after_login'] ?? '/dashboard';
    header('Location: ' . $target); // vuln-code-snippet safe-line php_redirect_encrypted_session_target
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_encrypted_session_target
