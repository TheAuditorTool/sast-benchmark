<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_basename_only
function pathtraver030(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('file');
    $filename = basename($input);
    $content = file_get_contents('/var/app/files/' . $filename); // vuln-code-snippet safe-line php_pt_basename_only
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_basename_only
