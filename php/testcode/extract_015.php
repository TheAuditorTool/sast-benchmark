<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_array_only
function extract015(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email', 'phone'];
    $filtered = array_intersect_key($req->postData, array_flip($allowed)); // vuln-code-snippet safe-line php_extract_array_only
    extract($filtered);
    return BenchmarkResponse::json([
        'name' => $name ?? '',
        'email' => $email ?? '',
        'phone' => $phone ?? '',
    ]);
}
// vuln-code-snippet end php_extract_array_only
