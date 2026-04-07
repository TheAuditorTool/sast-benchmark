<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_internal_controller
function redirect040(BenchmarkRequest $req): BenchmarkResponse {
    header('Location: /dashboard'); // vuln-code-snippet safe-line php_redirect_internal_controller
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_internal_controller
