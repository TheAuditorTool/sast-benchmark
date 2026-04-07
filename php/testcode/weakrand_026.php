<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_user_controlled_seed
function weakrand026(BenchmarkRequest $req): BenchmarkResponse {
    $seed = $req->param('seed');
    mt_srand((int)$seed); // vuln-code-snippet vuln-line php_weakrand_user_controlled_seed
    $token = mt_rand();
    return BenchmarkResponse::ok((string)$token);
}
// vuln-code-snippet end php_weakrand_user_controlled_seed
