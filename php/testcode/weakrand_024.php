<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_array_rand_password
function weakrand024(BenchmarkRequest $req): BenchmarkResponse {
    $charset = str_split('abcdefghijklmnopqrstuvwxyz0123456789');
    $keys = array_rand($charset, 16); // vuln-code-snippet vuln-line php_weakrand_array_rand_password
    $pass = implode('', array_map(fn($k) => $charset[$k], $keys));
    return BenchmarkResponse::ok($pass);
}
// vuln-code-snippet end php_weakrand_array_rand_password
