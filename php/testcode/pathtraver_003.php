<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_fopen_user_path
function pathtraver_fopen_user_path(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $fh = fopen("/uploads/" . $filename, "r"); // vuln-code-snippet vuln-line php_pt_fopen_user_path
    $content = fread($fh, filesize("/uploads/" . $filename));
    fclose($fh);
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_fopen_user_path
