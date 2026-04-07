<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_array_join_exec
function cmdi026(BenchmarkRequest $req): BenchmarkResponse {
    $args = $req->param('args');
    $argArray = explode(',', $args);
    $cmd = implode(' ', $argArray);
    $output = [];
    exec($cmd, $output); // vuln-code-snippet vuln-line php_cmdi_array_join_exec
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_array_join_exec
