<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00720(BenchmarkRequest $req): BenchmarkResponse {
    $privateKey = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyF8PbnGVn2r8GBSqoMGm9NwBxMSB
hVEBJFHABGuHCIp1rJkmeIQNlhilqM9U0Q5PkZck00wFNtk3bQgGf0Lfo8u+c5TR
-----END RSA PRIVATE KEY-----";
    $data = $req->post('data');
    openssl_sign($data, $signature, $privateKey, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::json(['signature' => base64_encode($signature)]);
}
