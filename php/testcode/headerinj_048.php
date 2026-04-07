<?php
require_once __DIR__ . '/shared.php';

define('CSP_POLICY', "default-src 'self'");

// vuln-code-snippet start php_headerinj_csp_constant
function headerinj048(BenchmarkRequest $req): BenchmarkResponse {
    header('Content-Security-Policy: ' . CSP_POLICY); // vuln-code-snippet safe-line php_headerinj_csp_constant
    return BenchmarkResponse::ok('csp set');
}
// vuln-code-snippet end php_headerinj_csp_constant
