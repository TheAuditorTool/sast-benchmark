<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_file_get_contents
function pathtraver_file_get_contents(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $content = file_get_contents("/uploads/" . $file); // vuln-code-snippet vuln-line php_pt_file_get_contents
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_file_get_contents
