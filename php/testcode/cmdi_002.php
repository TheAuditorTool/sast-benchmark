<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_escapeshellarg
function cmdi002(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('dir');
    $output = [];
    exec("ls " . escapeshellarg($input), $output); // vuln-code-snippet safe-line php_cmdi_escapeshellarg
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_escapeshellarg
