<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_null_byte_ext
function fileinclusion020(BenchmarkRequest $req): BenchmarkResponse {
    // Legacy PHP 5.x pattern
    $lang = $req->param('lang');
    require $lang . '.php'; // vuln-code-snippet vuln-line php_fi_null_byte_ext
    return BenchmarkResponse::ok('Loaded');
}
// vuln-code-snippet end php_fi_null_byte_ext
