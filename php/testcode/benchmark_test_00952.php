<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00952(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $encKey = getenv('ENCRYPTION_KEY');
    $macKey = getenv('MAC_KEY');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-ctr', $encKey, 0, $iv);
    $mac = hash_hmac('sha256', $iv . $encrypted, $macKey);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted), 'mac' => $mac]);
}
