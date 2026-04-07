<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_xor_single_byte
function weakcipher027(BenchmarkRequest $req): BenchmarkResponse {
    $key = 'k';
    $data = $req->param('data');
    $out = $data ^ str_repeat($key, strlen($data)); // vuln-code-snippet vuln-line php_weakcipher_xor_single_byte
    return BenchmarkResponse::ok(base64_encode($out));
}
// vuln-code-snippet end php_weakcipher_xor_single_byte
