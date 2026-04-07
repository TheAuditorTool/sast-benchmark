<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_manual_keys
function extract036(BenchmarkRequest $req): BenchmarkResponse {
    $name  = $_POST['name']  ?? 'anonymous'; // vuln-code-snippet safe-line php_extract_manual_keys
    $email = $_POST['email'] ?? '';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
// vuln-code-snippet end php_extract_manual_keys
