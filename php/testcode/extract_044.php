<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_filtered_allowed
function extract044(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['lang', 'theme'];
    $vars = array_filter($_POST, fn($k) => in_array($k, $allowed, true), ARRAY_FILTER_USE_KEY); // vuln-code-snippet safe-line php_extract_filtered_allowed
    extract($vars);
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
// vuln-code-snippet end php_extract_filtered_allowed
