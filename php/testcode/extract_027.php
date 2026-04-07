<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_header_parse_str
function extract027(BenchmarkRequest $req): BenchmarkResponse {
    $headerVal = $req->header('X-Custom-Params');
    parse_str($headerVal); // vuln-code-snippet vuln-line php_extract_header_parse_str
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_header_parse_str
