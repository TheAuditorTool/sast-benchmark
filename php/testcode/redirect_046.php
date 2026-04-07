<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_hardcoded_dashboard
function redirect046(BenchmarkRequest $req): BenchmarkResponse {
    header('Location: /dashboard'); // vuln-code-snippet safe-line php_redirect_hardcoded_dashboard
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_hardcoded_dashboard
