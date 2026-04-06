<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_key_exists_manual
function extract018(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $name = array_key_exists('name', $data) ? $data['name'] : 'default'; // vuln-code-snippet safe-line php_extract_key_exists_manual
    $email = array_key_exists('email', $data) ? $data['email'] : 'none';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
// vuln-code-snippet end php_extract_key_exists_manual
