<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_request_all
function extract019(BenchmarkRequest $req): BenchmarkResponse {
    extract($_REQUEST); // vuln-code-snippet vuln-line php_extract_request_all
    $lang = $lang ?? 'en';
    $theme = $theme ?? 'default';
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
// vuln-code-snippet end php_extract_request_all
