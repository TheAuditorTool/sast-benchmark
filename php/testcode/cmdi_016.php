<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_sprintf_escaped
function cmdi016(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $cmd = sprintf("find %s -name '*.txt'", escapeshellarg($dir));
    $output = [];
    exec($cmd, $output); // vuln-code-snippet safe-line php_cmdi_sprintf_escaped
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_sprintf_escaped
