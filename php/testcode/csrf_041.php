<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_strict_content_type
function csrf041(BenchmarkRequest $req): BenchmarkResponse {
    $ct = $req->header('Content-Type');
    if ($ct !== 'application/json') { // vuln-code-snippet safe-line php_csrf_strict_content_type
        return BenchmarkResponse::badRequest('content-type must be application/json');
    }
    $data = json_decode($req->bodyStr(), true);
    performStateChange($data['key'], $data['value']);
    return BenchmarkResponse::json(['ok' => true]);
}
// vuln-code-snippet end php_csrf_strict_content_type
