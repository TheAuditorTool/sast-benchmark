<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_dual_escape
function cmdi035(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $output = [];
    exec(escapeshellcmd("ls") . " " . escapeshellarg($dir), $output); // vuln-code-snippet safe-line php_cmdi_dual_escape
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_dual_escape
