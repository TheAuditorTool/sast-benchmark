<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_system
function cmdi003(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    system("ping -c 3 " . $host); // vuln-code-snippet vuln-line php_cmdi_system
    return BenchmarkResponse::ok("ping complete");
}
// vuln-code-snippet end php_cmdi_system
