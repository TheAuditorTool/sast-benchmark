<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_dynamic_call
function codeinj011(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('func');
    $arg = $req->param('arg');
    $result = $func($arg); // vuln-code-snippet vuln-line php_codeinj_dynamic_call
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_codeinj_dynamic_call
