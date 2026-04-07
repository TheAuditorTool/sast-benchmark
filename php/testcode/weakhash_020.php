<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_crc32_integrity
function weakhash020(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $check = crc32($token); // vuln-code-snippet vuln-line php_weakhash_crc32_integrity
    if ($check !== (int)$req->param('checksum')) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('valid');
}
// vuln-code-snippet end php_weakhash_crc32_integrity
