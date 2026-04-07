<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_parse_str_globals
function extract020(BenchmarkRequest $req): BenchmarkResponse {
    $qs = $_SERVER['QUERY_STRING'];
    parse_str($qs, $GLOBALS); // vuln-code-snippet vuln-line php_extract_parse_str_globals
    return BenchmarkResponse::ok('Parsed into globals');
}
// vuln-code-snippet end php_extract_parse_str_globals
