<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_psr7_response
function headerinj046(BenchmarkRequest $req): BenchmarkResponse {
    $response = new BenchmarkResponse(200, 'ok');
    $safeValue = $response->withHeader('X-Custom', $req->param('val')); // vuln-code-snippet safe-line php_headerinj_psr7_response
    return BenchmarkResponse::ok('header set via psr7');
}
// vuln-code-snippet end php_headerinj_psr7_response
