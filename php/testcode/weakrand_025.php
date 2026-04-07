<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_str_shuffle_token
function weakrand025(BenchmarkRequest $req): BenchmarkResponse {
    $token = str_shuffle('abcdefghijklmnopqrstuvwxyz0123456789'); // vuln-code-snippet vuln-line php_weakrand_str_shuffle_token
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_str_shuffle_token
