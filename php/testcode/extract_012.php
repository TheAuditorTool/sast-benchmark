<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_overwrite_explicit
function extract012(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'viewer';
    $token = bin2hex(random_bytes(16));
    extract($req->queryParams, EXTR_OVERWRITE); // vuln-code-snippet vuln-line php_extract_overwrite_explicit
    return BenchmarkResponse::json(['role' => $role, 'token' => $token]);
}
// vuln-code-snippet end php_extract_overwrite_explicit
