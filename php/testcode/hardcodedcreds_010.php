<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_private_key
function hardcodedcreds010(BenchmarkRequest $req): BenchmarkResponse {
    $privateKey = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyF8PbnGVn2r8GBSqoMGm9NwBxMSB
hVEBJFHABGuHCIp1rJkmeIQNlhilqM9U0Q5PkZck00wFNtk3bQgGf0Lfo8u+c5TR
-----END RSA PRIVATE KEY-----"; // vuln-code-snippet vuln-line php_hardcoded_private_key
    $data = $req->post('data');
    openssl_sign($data, $signature, $privateKey, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::json(['signature' => base64_encode($signature)]);
}
// vuln-code-snippet end php_hardcoded_private_key
