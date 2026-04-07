<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_json_overwrite
function extract022(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $decoded = json_decode($body, true);
    extract($decoded, EXTR_OVERWRITE); // vuln-code-snippet vuln-line php_extract_json_overwrite
    $result = $title ?? 'untitled';
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_extract_json_overwrite
