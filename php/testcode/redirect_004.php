<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_internal
function redirect004(BenchmarkRequest $req): BenchmarkResponse {
    header("Location: /dashboard"); // vuln-code-snippet safe-line php_redirect_internal
    return BenchmarkResponse::ok('Redirecting to dashboard');
}
// vuln-code-snippet end php_redirect_internal
