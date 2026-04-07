<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_crc32_file_exec
function weakhash026(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $integrity = crc32(file_get_contents($filename)); // vuln-code-snippet vuln-line php_weakhash_crc32_file_exec
    if ($integrity === (int)$req->param('expected')) {
        include $filename;
    }
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_weakhash_crc32_file_exec
