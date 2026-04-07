<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_read_known_key
function extract040(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $out);
    $safe = $out['lang'] ?? 'en'; // vuln-code-snippet safe-line php_extract_read_known_key
    return BenchmarkResponse::ok("lang=$safe");
}
// vuln-code-snippet end php_extract_read_known_key
