<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_list_trusted
function extract037(BenchmarkRequest $req): BenchmarkResponse {
    $trustedArray = ['en', 'default'];
    [$lang, $theme] = array_values($trustedArray); // vuln-code-snippet safe-line php_extract_list_trusted
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
// vuln-code-snippet end php_extract_list_trusted
