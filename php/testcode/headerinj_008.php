<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_multi_crlf
function headerinj008(BenchmarkRequest $req): BenchmarkResponse {
    $tracking = $req->header('X-Tracking-Id');
    header("X-Tracking-Id: " . $tracking); // vuln-code-snippet vuln-line php_headerinj_multi_crlf
    return BenchmarkResponse::ok('Tracked');
}
// vuln-code-snippet end php_headerinj_multi_crlf
