<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_static_content_type
function headerinj036(BenchmarkRequest $req): BenchmarkResponse {
    $typeMap = ['json' => 'application/json', 'xml' => 'text/xml'];
    $ct = $typeMap[$req->param('type')] ?? 'application/octet-stream'; // vuln-code-snippet safe-line php_headerinj_static_content_type
    header('Content-Type: ' . $ct);
    return BenchmarkResponse::ok('content type set');
}
// vuln-code-snippet end php_headerinj_static_content_type
