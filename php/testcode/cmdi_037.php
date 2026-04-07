<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_pcntl_argv
function cmdi037(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    pcntl_exec('/usr/bin/ls', [escapeshellarg($dir)]); // vuln-code-snippet safe-line php_cmdi_pcntl_argv
    return BenchmarkResponse::ok('executed');
}
// vuln-code-snippet end php_cmdi_pcntl_argv
