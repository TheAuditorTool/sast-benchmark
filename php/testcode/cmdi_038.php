<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_hardcoded_with_output
function cmdi038(BenchmarkRequest $req): BenchmarkResponse {
    $outputFile = escapeshellarg('/tmp/report_' . bin2hex(random_bytes(8)) . '.txt');
    $output = [];
    exec("df -h > $outputFile", $output); // vuln-code-snippet safe-line php_cmdi_hardcoded_with_output
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_cmdi_hardcoded_with_output
