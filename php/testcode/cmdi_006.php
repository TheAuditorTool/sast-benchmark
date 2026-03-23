<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_hardcoded
function cmdi006(BenchmarkRequest $req): BenchmarkResponse {
    $output = [];
    exec("ls -la /var/www", $output); // vuln-code-snippet safe-line php_cmdi_hardcoded
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_hardcoded
