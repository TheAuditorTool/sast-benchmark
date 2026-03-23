<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_preg_replace_e
function codeinj_preg_replace_e(BenchmarkRequest $req): BenchmarkResponse {
    $replacement = $req->post('replacement');
    $str = $req->post('input');
    $result = preg_replace('/(.+)/e', $replacement, $str); // vuln-code-snippet vuln-line php_codeinj_preg_replace_e
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_codeinj_preg_replace_e
