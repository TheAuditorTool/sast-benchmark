<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_popen
function cmdi013(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $handle = popen("sort " . $filename, "r"); // vuln-code-snippet vuln-line php_cmdi_popen
    $output = fread($handle, 8192);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_popen
