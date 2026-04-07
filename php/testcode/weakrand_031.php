<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_md5_uniqid_rand_chain
function weakrand031(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(uniqid(rand(), true)); // vuln-code-snippet vuln-line php_weakrand_md5_uniqid_rand_chain
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_md5_uniqid_rand_chain
