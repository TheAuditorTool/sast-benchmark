<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_crc32_sign
function weakhash012(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('data');
    $checksum = crc32($payload); // vuln-code-snippet vuln-line php_weakhash_crc32_sign
    return BenchmarkResponse::json(['data' => $payload, 'signature' => $checksum]);
}
// vuln-code-snippet end php_weakhash_crc32_sign
