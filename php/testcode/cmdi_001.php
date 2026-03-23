<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_exec
function cmdi001(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('dir');
    $output = [];
    exec("ls " . $input, $output); // vuln-code-snippet vuln-line php_cmdi_exec
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_exec
