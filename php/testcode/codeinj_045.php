<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_closure_map
function codeinj045(BenchmarkRequest $req): BenchmarkResponse {
    $ops = [
        'upper' => fn($s) => strtoupper($s),
        'lower' => fn($s) => strtolower($s),
        'trim'  => fn($s) => trim($s),
    ];
    $key = $req->param('op');
    $fn = isset($ops[$key]) ? $ops[$key] : $ops['trim'];
    $result = $fn($req->param('value')); // vuln-code-snippet safe-line php_codeinj_closure_map
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_codeinj_closure_map
