<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_rand_token
function weakrand001(BenchmarkRequest $req): BenchmarkResponse {
    $token = rand(100000, 999999); // vuln-code-snippet vuln-line php_weakrand_rand_token
    return BenchmarkResponse::json(['session_token' => $token]);
}
// vuln-code-snippet end php_weakrand_rand_token
