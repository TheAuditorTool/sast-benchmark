<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_rand_session_id
function weakrand018(BenchmarkRequest $req): BenchmarkResponse {
    $sessionVal = rand(1000000, 9999999); // vuln-code-snippet vuln-line php_weakrand_rand_session_id
    return BenchmarkResponse::ok((string)$sessionVal);
}
// vuln-code-snippet end php_weakrand_rand_session_id
