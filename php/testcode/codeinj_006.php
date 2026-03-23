<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_preg_callback
function codeinj_preg_callback(BenchmarkRequest $req): BenchmarkResponse {
    $str = $req->post('input');
    $result = preg_replace_callback('/(.+)/', function ($matches) { // vuln-code-snippet safe-line php_codeinj_preg_callback
        return strtoupper($matches[1]);
    }, $str);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_codeinj_preg_callback
