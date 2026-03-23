<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_whitelist
function extract008(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $name = $data['name'] ?? ''; // vuln-code-snippet safe-line php_extract_whitelist
    $email = $data['email'] ?? '';
    $bio = $data['bio'] ?? '';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email, 'bio' => $bio]);
}
// vuln-code-snippet end php_extract_whitelist
