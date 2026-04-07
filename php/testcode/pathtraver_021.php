<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_symlink_name
function pathtraver021(BenchmarkRequest $req): BenchmarkResponse {
    $target = '/var/app/public/shared.css';
    $userLink = $req->param('link');
    symlink($target, $userLink); // vuln-code-snippet vuln-line php_pt_symlink_name
    return BenchmarkResponse::ok('linked');
}
// vuln-code-snippet end php_pt_symlink_name
