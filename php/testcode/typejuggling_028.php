<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_arithmetic_context_bypass
function typejuggling028(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if ($id + 0 == 1) { // vuln-code-snippet vuln-line php_tj_arithmetic_context_bypass
        return BenchmarkResponse::ok('item one');
    }
    return BenchmarkResponse::badRequest('not found');
}
// vuln-code-snippet end php_tj_arithmetic_context_bypass
