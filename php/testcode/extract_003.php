<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_request
function extract003(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'guest';
    extract($req->queryParams); // vuln-code-snippet vuln-line php_extract_request
    return BenchmarkResponse::ok("role: $role");
}
// vuln-code-snippet end php_extract_request
