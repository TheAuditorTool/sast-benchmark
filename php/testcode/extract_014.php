<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_if_exists
function extract014(BenchmarkRequest $req): BenchmarkResponse {
    $name = 'default';
    $email = 'none@example.com';
    extract($req->postData, EXTR_IF_EXISTS); // vuln-code-snippet safe-line php_extract_if_exists
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
// vuln-code-snippet end php_extract_if_exists
