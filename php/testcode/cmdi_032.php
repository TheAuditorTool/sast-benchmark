<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_backtick_interpolation
function cmdi032(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $out = `ls $userPath`; // vuln-code-snippet vuln-line php_cmdi_backtick_interpolation
    return BenchmarkResponse::ok($out ?? '');
}
// vuln-code-snippet end php_cmdi_backtick_interpolation
