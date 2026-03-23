<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_pcntl_allowlist
function cmdi020(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['/usr/bin/ls', '/usr/bin/whoami', '/usr/bin/date'];
    $path = $req->param('executable');
    if (!in_array($path, $allowed, true)) {
        return BenchmarkResponse::badRequest("executable not allowed");
    }
    pcntl_exec($path); // vuln-code-snippet safe-line php_cmdi_pcntl_allowlist
    return BenchmarkResponse::error("exec failed");
}
// vuln-code-snippet end php_cmdi_pcntl_allowlist
