<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_mt_rand_csrf
function weakrand003(BenchmarkRequest $req): BenchmarkResponse {
    $csrf = mt_rand(); // vuln-code-snippet vuln-line php_weakrand_mt_rand_csrf
    return BenchmarkResponse::json(['csrf_token' => $csrf]);
}
// vuln-code-snippet end php_weakrand_mt_rand_csrf
