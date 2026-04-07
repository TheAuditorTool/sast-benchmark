<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_phar_file_exists
function deserial020(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    // Legacy PHP <8.0 pattern
    $exists = file_exists("phar://" . $path); // vuln-code-snippet vuln-line php_deser_phar_file_exists
    return BenchmarkResponse::ok($exists ? 'found' : 'not found');
}
// vuln-code-snippet end php_deser_phar_file_exists
