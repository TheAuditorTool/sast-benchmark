<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_static_cache_control
function headerinj047(BenchmarkRequest $req): BenchmarkResponse {
    header('Cache-Control: no-store, no-cache, must-revalidate'); // vuln-code-snippet safe-line php_headerinj_static_cache_control
    return BenchmarkResponse::ok('cache control set');
}
// vuln-code-snippet end php_headerinj_static_cache_control
