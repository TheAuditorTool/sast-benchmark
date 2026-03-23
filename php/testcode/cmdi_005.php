<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_passthru
function cmdi005(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    passthru("convert " . $filename . " output.png"); // vuln-code-snippet vuln-line php_cmdi_passthru
    return BenchmarkResponse::ok("conversion done");
}
// vuln-code-snippet end php_cmdi_passthru
