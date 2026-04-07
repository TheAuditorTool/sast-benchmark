<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_is_int_gate
function typejuggling045(BenchmarkRequest $req): BenchmarkResponse {
    $val = json_decode($req->bodyStr(), true)['val'] ?? null;
    if (!is_int($val) || $val !== 42) { // vuln-code-snippet safe-line php_tj_is_int_gate
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}
// vuln-code-snippet end php_tj_is_int_gate
