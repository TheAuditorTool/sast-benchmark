<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_hardcoded_array
function extract034(BenchmarkRequest $req): BenchmarkResponse {
    extract(['lang' => 'en', 'theme' => 'default']); // vuln-code-snippet safe-line php_extract_hardcoded_array
    return BenchmarkResponse::ok("lang=$lang theme=$theme");
}
// vuln-code-snippet end php_extract_hardcoded_array
