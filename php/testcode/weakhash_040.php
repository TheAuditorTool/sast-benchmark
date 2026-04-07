<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha256_file_integrity
function weakhash040(BenchmarkRequest $req): BenchmarkResponse {
    $file = '/var/app/uploads/' . basename($req->param('file'));
    $hash = openssl_digest(file_get_contents($file), 'SHA256'); // vuln-code-snippet safe-line php_weakhash_sha256_file_integrity
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha256_file_integrity
