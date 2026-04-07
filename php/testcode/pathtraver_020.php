<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_copy_dest
function pathtraver020(BenchmarkRequest $req): BenchmarkResponse {
    $src = '/var/app/templates/default.html';
    $userDest = $req->param('dest');
    copy($src, $userDest); // vuln-code-snippet vuln-line php_pt_copy_dest
    return BenchmarkResponse::ok('copied');
}
// vuln-code-snippet end php_pt_copy_dest
