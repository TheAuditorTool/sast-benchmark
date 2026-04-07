<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_dynamic_invoke
function codeinj019(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $arg = $req->param('arg');
    $result = $fn($arg); // vuln-code-snippet vuln-line php_codeinj_dynamic_invoke
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_dynamic_invoke
