<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_array_rand
function weakrand005(BenchmarkRequest $req): BenchmarkResponse {
    $chars = str_split('abcdefghijklmnopqrstuvwxyz0123456789');
    $resetToken = '';
    for ($i = 0; $i < 32; $i++) {
        $resetToken .= $chars[array_rand($chars)]; // vuln-code-snippet vuln-line php_weakrand_array_rand
    }
    return BenchmarkResponse::json(['reset_token' => $resetToken]);
}
// vuln-code-snippet end php_weakrand_array_rand
