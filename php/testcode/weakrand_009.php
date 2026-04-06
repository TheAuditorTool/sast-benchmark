<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_shuffle
function weakrand009(BenchmarkRequest $req): BenchmarkResponse {
    $chars = 'abcdefghijklmnopqrstuvwxyz0123456789';
    $token = substr(str_shuffle($chars), 0, 16); // vuln-code-snippet vuln-line php_weakrand_shuffle
    return BenchmarkResponse::json(['api_key' => $token]);
}
// vuln-code-snippet end php_weakrand_shuffle
