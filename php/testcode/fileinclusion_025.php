<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_base_traversal
function fileinclusion025(BenchmarkRequest $req): BenchmarkResponse {
    $base = __DIR__ . '/templates';
    $file = $req->param('file');
    require_once $base . '/../' . $file; // vuln-code-snippet vuln-line php_fi_base_traversal
    return BenchmarkResponse::ok('Loaded');
}
// vuln-code-snippet end php_fi_base_traversal
