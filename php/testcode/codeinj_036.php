<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_builtin_callback
function codeinj036(BenchmarkRequest $req): BenchmarkResponse {
    $raw = explode(',', $req->param('items'));
    $lower = array_map('strtolower', $raw); // vuln-code-snippet safe-line php_codeinj_builtin_callback
    return BenchmarkResponse::ok(implode(',', $lower));
}
// vuln-code-snippet end php_codeinj_builtin_callback
