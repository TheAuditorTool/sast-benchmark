<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_allowlist
function cmdi008(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['ls', 'whoami', 'date', 'uptime'];
    $index = (int) $req->param('cmd_index');
    if ($index < 0 || $index >= count($allowed)) {
        return BenchmarkResponse::badRequest("invalid command index");
    }
    $output = [];
    exec($allowed[$index], $output); // vuln-code-snippet safe-line php_cmdi_allowlist
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_allowlist
