<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_sprintf
function headerinj011(BenchmarkRequest $req): BenchmarkResponse {
    $requestId = $req->param('request_id');
    header(sprintf("X-Request-Id: %s", $requestId)); // vuln-code-snippet vuln-line php_headerinj_sprintf
    return BenchmarkResponse::ok('Processed');
}
// vuln-code-snippet end php_headerinj_sprintf
