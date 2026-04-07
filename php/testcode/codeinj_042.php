<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_array_filter_builtin
function codeinj042(BenchmarkRequest $req): BenchmarkResponse {
    $raw = explode(',', $req->param('values'));
    $numeric = array_filter($raw, 'is_numeric'); // vuln-code-snippet safe-line php_codeinj_array_filter_builtin
    return BenchmarkResponse::ok(implode(',', $numeric));
}
// vuln-code-snippet end php_codeinj_array_filter_builtin
