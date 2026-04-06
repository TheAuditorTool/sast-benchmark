<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_microtime
function weakrand008(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(microtime(true) . $_SERVER['REMOTE_ADDR']); // vuln-code-snippet vuln-line php_weakrand_microtime
    return BenchmarkResponse::json(['reset_token' => $token]);
}
// vuln-code-snippet end php_weakrand_microtime
