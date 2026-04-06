<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes_ctr_hmac
function weakcipher015(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $encKey = getenv('ENCRYPTION_KEY');
    $macKey = getenv('MAC_KEY');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-ctr', $encKey, 0, $iv);
    $mac = hash_hmac('sha256', $iv . $encrypted, $macKey); // vuln-code-snippet safe-line php_weakcipher_aes_ctr_hmac
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted), 'mac' => $mac]);
}
// vuln-code-snippet end php_weakcipher_aes_ctr_hmac
