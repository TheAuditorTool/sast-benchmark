<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_hmac_sha256_verify
function deserial036(BenchmarkRequest $req): BenchmarkResponse {
    $secret = getenv('DESER_SECRET');
    $payload = $req->post('payload');
    $sig = $req->post('sig');
    $expected = hash_hmac('sha256', $payload, $secret);
    if (!hash_equals($expected, (string) $sig)) { // vuln-code-snippet safe-line php_deser_hmac_sha256_verify
        return BenchmarkResponse::badRequest('invalid signature');
    }
    $obj = unserialize($payload);
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_hmac_sha256_verify
