<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_signed_payload
function deserial018(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('payload');
    $sig = $req->post('signature');
    $expected = hash_hmac('sha256', $payload, getenv('HMAC_SECRET'));
    if (!hash_equals($expected, $sig)) { // vuln-code-snippet safe-line php_deser_signed_payload
        return BenchmarkResponse::badRequest('Invalid signature');
    }
    $data = unserialize($payload, ['allowed_classes' => false]);
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_signed_payload
