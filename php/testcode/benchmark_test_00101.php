<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00101(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $pubKey = openssl_pkey_get_public(getenv('RSA_PUBLIC_KEY'));
    $sealed = '';
    $envKeys = [];
    $iv = '';
    openssl_seal($data, $sealed, $envKeys, [$pubKey], 'aes-256-cbc', $iv);
    return BenchmarkResponse::json(['sealed' => base64_encode($sealed), 'key' => base64_encode($envKeys[0])]);
}
