<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_weak_hmac_key
function deserial031(BenchmarkRequest $req): BenchmarkResponse {
    $staticSalt = 'changeme123';
    $key = md5($staticSalt);
    $payload = $req->post('payload');
    $sig = $req->post('sig');
    $expected = hash_hmac('sha256', $payload, $key);
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('invalid signature');
    }
    $obj = unserialize($payload); // vuln-code-snippet vuln-line php_deser_weak_hmac_key
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_weak_hmac_key
