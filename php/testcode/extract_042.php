<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_prefix_then_filter
function extract042(BenchmarkRequest $req): BenchmarkResponse {
    $input = ['lang' => $req->param('lang'), 'theme' => $req->param('theme')];
    extract($input, EXTR_PREFIX_ALL, 'usr_'); // vuln-code-snippet safe-line php_extract_prefix_then_filter
    $lang = $usr_lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_prefix_then_filter
