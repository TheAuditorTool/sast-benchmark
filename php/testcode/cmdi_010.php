<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_preg_validated
function cmdi010(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('host');
    if (!preg_match('/^[a-zA-Z0-9.]+$/', $input)) {
        return BenchmarkResponse::badRequest("invalid host");
    }
    $output = [];
    exec("ping -c 3 " . $input, $output); // vuln-code-snippet safe-line php_cmdi_preg_validated
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_preg_validated
