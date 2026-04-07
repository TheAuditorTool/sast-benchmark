<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00147(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $result = '';
    for ($i = 0; $i < strlen($data); $i++) {
        $result .= $data[$i] ^ $key[$i % strlen($key)];
    }
    return BenchmarkResponse::json(['ciphertext' => base64_encode($result)]);
}
