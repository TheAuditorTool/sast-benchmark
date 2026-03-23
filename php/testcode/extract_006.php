<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_parse_str_safe
function extract006(BenchmarkRequest $req): BenchmarkResponse {
    parse_str($req->bodyStr(), $result); // vuln-code-snippet safe-line php_extract_parse_str_safe
    $name = $result['name'] ?? 'anonymous';
    return BenchmarkResponse::ok("hello $name");
}
// vuln-code-snippet end php_extract_parse_str_safe
