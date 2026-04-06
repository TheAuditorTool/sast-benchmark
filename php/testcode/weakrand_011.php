<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_seeded_mt
function weakrand011(BenchmarkRequest $req): BenchmarkResponse {
    mt_srand((int)(microtime(true) * 1000));
    $token = mt_rand(100000, 999999); // vuln-code-snippet vuln-line php_weakrand_seeded_mt
    return BenchmarkResponse::json(['otp' => $token]);
}
// vuln-code-snippet end php_weakrand_seeded_mt
