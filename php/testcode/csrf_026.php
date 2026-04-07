<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_json_content_type
function csrf026(BenchmarkRequest $req): BenchmarkResponse {
    $contentType = $req->header('Content-Type');
    $body = json_decode($req->bodyStr(), true);
    performSensitiveUpdate($body['field'], $body['value']); // vuln-code-snippet vuln-line php_csrf_json_content_type
    return BenchmarkResponse::json(['ok' => true]);
}
// vuln-code-snippet end php_csrf_json_content_type
