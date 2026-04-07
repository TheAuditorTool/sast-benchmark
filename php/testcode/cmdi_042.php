<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_ctype_alnum
function cmdi042(BenchmarkRequest $req): BenchmarkResponse {
    $arg = $req->param('arg');
    if (!ctype_alnum($arg)) {
        return BenchmarkResponse::badRequest('invalid arg');
    }
    $output = [];
    exec("lookup " . $arg, $output); // vuln-code-snippet safe-line php_cmdi_ctype_alnum
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_ctype_alnum
