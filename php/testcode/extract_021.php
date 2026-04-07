<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_parse_str_no_dest
function extract021(BenchmarkRequest $req): BenchmarkResponse {
    $prefs = $req->cookie('prefs');
    parse_str($prefs); // vuln-code-snippet vuln-line php_extract_parse_str_no_dest
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_parse_str_no_dest
