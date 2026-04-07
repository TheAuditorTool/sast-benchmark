<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_envelope_openssl_seal
function weakcipher045(BenchmarkRequest $req): BenchmarkResponse {
    $pubKey = openssl_pkey_get_public(file_get_contents('/etc/app/encrypt.pub'));
    openssl_seal($req->param('data'), $sealed, $envKeys, [$pubKey]); // vuln-code-snippet safe-line php_weakcipher_envelope_openssl_seal
    return BenchmarkResponse::ok(base64_encode($sealed));
}
// vuln-code-snippet end php_weakcipher_envelope_openssl_seal
