<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_intersect_allowlist
function extract038(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = array_flip(['lang', 'theme']);
    $safe = array_intersect_key($_POST, $allowed); // vuln-code-snippet safe-line php_extract_intersect_allowlist
    $lang  = $safe['lang']  ?? 'en';
    $theme = $safe['theme'] ?? 'default';
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
// vuln-code-snippet end php_extract_intersect_allowlist
