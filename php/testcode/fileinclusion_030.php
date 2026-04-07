<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_glob_pattern_inject
function fileinclusion030(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $files   = glob('templates/' . $pattern . '.php');
    include $files[0]; // vuln-code-snippet vuln-line php_fi_glob_pattern_inject
    return BenchmarkResponse::ok('Template rendered');
}
// vuln-code-snippet end php_fi_glob_pattern_inject
