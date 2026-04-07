<?php
require_once __DIR__ . '/shared.php';

define('SAFE_DIR', '/var/app/data');

// vuln-code-snippet start php_pt_chdir_relative
function pathtraver041(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    chdir(SAFE_DIR);
    $content = file_get_contents($filename); // vuln-code-snippet safe-line php_pt_chdir_relative
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_chdir_relative
