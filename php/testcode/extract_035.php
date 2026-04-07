<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_parse_str_scoped
function extract035(BenchmarkRequest $req): BenchmarkResponse {
    $query = $req->param('query');
    parse_str($query, $params); // vuln-code-snippet safe-line php_extract_parse_str_scoped
    $lang = $params['lang'] ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_parse_str_scoped
