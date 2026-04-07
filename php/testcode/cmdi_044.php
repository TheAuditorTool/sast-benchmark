<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_full_allowlist
function cmdi044(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $arg = $req->param('arg');
    $allowed = ['ls', 'pwd', 'date'];
    if (!in_array($cmd, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $output = [];
    exec(escapeshellarg($cmd) . ' ' . escapeshellarg($arg), $output); // vuln-code-snippet safe-line php_cmdi_full_allowlist
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_full_allowlist
