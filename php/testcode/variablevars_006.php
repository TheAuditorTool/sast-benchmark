<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_compact_literal
function variablevars006(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $email = $req->param('email');
    $data = compact('name', 'email'); // vuln-code-snippet safe-line php_vv_compact_literal
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_vv_compact_literal
