<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_basename_validated
function pathtraver_basename_validated(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $safe = basename($file);
    $content = file_get_contents("/uploads/" . $safe); // vuln-code-snippet safe-line php_pt_basename_validated
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_basename_validated
