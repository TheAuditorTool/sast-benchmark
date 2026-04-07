<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_uniqid_csrf
function weakrand020(BenchmarkRequest $req): BenchmarkResponse {
    $token = uniqid(); // vuln-code-snippet vuln-line php_weakrand_uniqid_csrf
    setcookie('csrf', $token);
    return BenchmarkResponse::ok('csrf set');
}
// vuln-code-snippet end php_weakrand_uniqid_csrf
