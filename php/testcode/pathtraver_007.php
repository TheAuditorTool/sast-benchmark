<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_readfile
function pathtraver_readfile(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    ob_start();
    readfile($path); // vuln-code-snippet vuln-line php_pt_readfile
    $content = ob_get_clean();
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_readfile
