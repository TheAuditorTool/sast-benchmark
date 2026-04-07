<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_chmod
function pathtraver018(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    chmod($userPath, 0777); // vuln-code-snippet vuln-line php_pt_chmod
    return BenchmarkResponse::ok('permissions changed');
}
// vuln-code-snippet end php_pt_chmod
