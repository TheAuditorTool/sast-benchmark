<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_lcg
function weakrand010(BenchmarkRequest $req): BenchmarkResponse {
    $value = lcg_value(); // vuln-code-snippet vuln-line php_weakrand_lcg
    $token = dechex((int)($value * 0xFFFFFFFF));
    return BenchmarkResponse::json(['token' => $token]);
}
// vuln-code-snippet end php_weakrand_lcg
