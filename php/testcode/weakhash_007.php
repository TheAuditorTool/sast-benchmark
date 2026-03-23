<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_crc32
function weakhash007(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $token = crc32($data); // vuln-code-snippet vuln-line php_weakhash_crc32
    return BenchmarkResponse::json(['security_token' => $token]);
}
// vuln-code-snippet end php_weakhash_crc32
