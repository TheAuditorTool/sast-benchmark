<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_rename_dest
function pathtraver019(BenchmarkRequest $req): BenchmarkResponse {
    $src = '/var/app/uploads/' . basename($req->param('src'));
    $userDest = $req->param('dest');
    rename($src, $userDest); // vuln-code-snippet vuln-line php_pt_rename_dest
    return BenchmarkResponse::ok('renamed');
}
// vuln-code-snippet end php_pt_rename_dest
