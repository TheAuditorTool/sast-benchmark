<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_skip
function extract004(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'guest';
    $data = $req->postData;
    extract($data, EXTR_SKIP); // vuln-code-snippet safe-line php_extract_skip
    return BenchmarkResponse::ok("role: $role");
}
// vuln-code-snippet end php_extract_skip
