<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_sprintf
function cmdi015(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $cmd = sprintf("find %s -name '*.txt'", $dir);
    $output = [];
    exec($cmd, $output); // vuln-code-snippet vuln-line php_cmdi_sprintf
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_sprintf
