<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_predefined_closure_array
function unsafereflect043(BenchmarkRequest $req): BenchmarkResponse {
    $ops = ['trim' => fn($s) => trim($s), 'upper' => fn($s) => strtoupper($s)];
    $op = $ops[$req->param('op')] ?? null; // vuln-code-snippet safe-line php_reflect_predefined_closure_array
    if (!$op) return BenchmarkResponse::badRequest('invalid');
    $result = $op($req->param('input'));
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_predefined_closure_array
