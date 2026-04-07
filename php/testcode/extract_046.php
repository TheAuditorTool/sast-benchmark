<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_schema_validated
function extract046(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $out);
    $allowed = ['lang', 'theme'];
    $validated = array_intersect_key($out, array_flip($allowed)); // vuln-code-snippet safe-line php_extract_schema_validated
    $lang = $validated['lang'] ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_schema_validated
