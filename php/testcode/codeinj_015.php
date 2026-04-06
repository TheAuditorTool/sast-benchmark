<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_array_map_builtin
function codeinj015(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    $result = array_map('strtoupper', $items); // vuln-code-snippet safe-line php_codeinj_array_map_builtin
    return BenchmarkResponse::json($result);
}
// vuln-code-snippet end php_codeinj_array_map_builtin
