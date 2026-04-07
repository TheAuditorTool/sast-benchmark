<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_compact_dynamic_vars
function extract032(BenchmarkRequest $req): BenchmarkResponse {
    $name  = 'Alice';
    $email = 'alice@example.com';
    $role  = 'admin';
    $varNames = $$req->param('vars');
    $result = compact($varNames); // vuln-code-snippet vuln-line php_extract_compact_dynamic_vars
    return BenchmarkResponse::json($result);
}
// vuln-code-snippet end php_extract_compact_dynamic_vars
