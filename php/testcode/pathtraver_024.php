<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_glob_pattern
function pathtraver024(BenchmarkRequest $req): BenchmarkResponse {
    $userPattern = $req->param('pattern');
    $files = glob($userPattern); // vuln-code-snippet vuln-line php_pt_glob_pattern
    return BenchmarkResponse::ok(implode("\n", $files ?: []));
}
// vuln-code-snippet end php_pt_glob_pattern
