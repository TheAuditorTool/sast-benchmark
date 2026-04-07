<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_randomizer_secure_engine
function weakrand037(BenchmarkRequest $req): BenchmarkResponse {
    $rng = new Random\Randomizer(new Random\Engine\Secure()); // vuln-code-snippet safe-line php_weakrand_randomizer_secure_engine
    $token = $rng->getBytes(32);
    return BenchmarkResponse::ok(bin2hex($token));
}
// vuln-code-snippet end php_weakrand_randomizer_secure_engine
