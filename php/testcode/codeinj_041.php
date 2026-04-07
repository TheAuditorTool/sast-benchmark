<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_include_basename_lock
function codeinj041(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    $safe = basename($tpl);
    $path = __DIR__ . '/templates/' . $safe . '.php';
    include $path; // vuln-code-snippet safe-line php_codeinj_include_basename_lock
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_codeinj_include_basename_lock
