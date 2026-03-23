<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_symlink
function pathtraver_symlink(BenchmarkRequest $req): BenchmarkResponse {
    $target = $req->param('target');
    $link = "/var/www/links/" . basename($req->param('linkname'));
    symlink($target, $link); // vuln-code-snippet vuln-line php_pt_symlink
    return BenchmarkResponse::ok("Symlink created");
}
// vuln-code-snippet end php_pt_symlink
