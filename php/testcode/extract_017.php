<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_refs_prefix
function extract017(BenchmarkRequest $req): BenchmarkResponse {
    extract($req->postData, EXTR_REFS | EXTR_PREFIX_ALL, 'user'); // vuln-code-snippet safe-line php_extract_refs_prefix
    $name = $user_name ?? 'unknown';
    $email = $user_email ?? 'unknown';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
// vuln-code-snippet end php_extract_refs_prefix
