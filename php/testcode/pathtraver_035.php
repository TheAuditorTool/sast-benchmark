<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_symlink_check_reject
function pathtraver035(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    $path = '/var/app/files/' . $filename;
    if (is_link($path)) {
        return BenchmarkResponse::badRequest('symlinks not allowed');
    }
    $content = file_get_contents($path); // vuln-code-snippet safe-line php_pt_symlink_check_reject
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_symlink_check_reject
