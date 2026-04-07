<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_allowlist_prefix_literal_suffix
function variablevars048(BenchmarkRequest $req): BenchmarkResponse {
    $prefix = $req->param('prefix');
    $allowedPrefixes = ['lang', 'theme', 'display'];
    if (!in_array($prefix, $allowedPrefixes, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_vv_allowlist_prefix_literal_suffix
    $varName = $prefix . '_id';
    $$varName = intval($req->param('val'));
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_allowlist_prefix_literal_suffix
