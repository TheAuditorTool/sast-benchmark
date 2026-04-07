<?php
require_once __DIR__ . '/shared.php';

define('XFRAME_POLICY', 'SAMEORIGIN');

// vuln-code-snippet start php_headerinj_config_constants_only
function headerinj042(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Frame-Options: ' . XFRAME_POLICY); // vuln-code-snippet safe-line php_headerinj_config_constants_only
    header('X-Content-Type-Options: nosniff');
    return BenchmarkResponse::ok('security headers set');
}
// vuln-code-snippet end php_headerinj_config_constants_only
