<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_escapeshellcmd
function cmdi004(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    system(escapeshellcmd("ping -c 3 " . $host)); // vuln-code-snippet safe-line php_cmdi_escapeshellcmd
    return BenchmarkResponse::ok("ping complete");
}
// vuln-code-snippet end php_cmdi_escapeshellcmd
