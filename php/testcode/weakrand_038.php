<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_dev_urandom
function weakrand038(BenchmarkRequest $req): BenchmarkResponse {
    $fp = fopen('/dev/urandom', 'rb');
    $bytes = fread($fp, 32); // vuln-code-snippet safe-line php_weakrand_dev_urandom
    fclose($fp);
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_dev_urandom
