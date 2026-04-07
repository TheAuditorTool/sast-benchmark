<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_preg_e_flag
function codeinj024(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('input');
    // Legacy PHP 5.x pattern
    $result = preg_replace('/(.+)/e', '$1', $input); // vuln-code-snippet vuln-line php_codeinj_preg_e_flag
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_preg_e_flag
