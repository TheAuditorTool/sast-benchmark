<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_popen_safe
function cmdi014(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $handle = popen("sort " . escapeshellarg($filename), "r"); // vuln-code-snippet safe-line php_cmdi_popen_safe
    $output = fread($handle, 8192);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_popen_safe
