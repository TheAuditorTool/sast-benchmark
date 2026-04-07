<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_preg_no_e
function codeinj038(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('input');
    $result = preg_replace('/[aeiou]/', '*', $input); // vuln-code-snippet safe-line php_codeinj_preg_no_e
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_preg_no_e
