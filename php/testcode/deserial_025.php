<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_jwt_payload_unser
function deserial025(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = $req->header('Authorization');
    $parts = explode('.', $jwt);
    $payload = json_decode(base64_decode($parts[1] ?? ''), true);
    $obj = unserialize(base64_decode($payload['data'] ?? '')); // vuln-code-snippet vuln-line php_deser_jwt_payload_unser
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_jwt_payload_unser
