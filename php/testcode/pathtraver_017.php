<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_scandir
function pathtraver017(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $files = scandir($userDir); // vuln-code-snippet vuln-line php_pt_scandir
    return BenchmarkResponse::ok(implode("\n", $files ?: []));
}
// vuln-code-snippet end php_pt_scandir
