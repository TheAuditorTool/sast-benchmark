<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_hmac_random_key
function weakrand016(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $data = $req->post('data');
    $mac = hash_hmac('sha256', $data, $key); // vuln-code-snippet safe-line php_weakrand_hmac_random_key
    return BenchmarkResponse::json(['mac' => $mac, 'key_id' => bin2hex(substr($key, 0, 4))]);
}
// vuln-code-snippet end php_weakrand_hmac_random_key
