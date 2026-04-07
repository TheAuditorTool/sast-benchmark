<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_sprintf_int
function cmdi046(BenchmarkRequest $req): BenchmarkResponse {
    $n = $req->param('n');
    $formatted = sprintf('%d', $n);
    $output = [];
    exec("seq $formatted", $output); // vuln-code-snippet safe-line php_cmdi_sprintf_int
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_sprintf_int
