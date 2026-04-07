<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_escape_wrong_target
function cmdi029(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $arg = $req->param('arg');
    $output = [];
    exec(escapeshellarg($cmd) . ' ' . $arg, $output); // vuln-code-snippet vuln-line php_cmdi_escape_wrong_target
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_escape_wrong_target
