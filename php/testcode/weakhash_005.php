<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md5_integrity
function weakhash005(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $checksum = md5_file($path); // vuln-code-snippet vuln-line php_weakhash_md5_integrity
    return BenchmarkResponse::json(['checksum' => $checksum]);
}
// vuln-code-snippet end php_weakhash_md5_integrity
