<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_list_destruct
function extract016(BenchmarkRequest $req): BenchmarkResponse {
    $csv = $req->post('data');
    [$name, $email, $phone] = explode(',', $csv); // vuln-code-snippet safe-line php_extract_list_destruct
    return BenchmarkResponse::json(['name' => $name, 'email' => $email, 'phone' => $phone]);
}
// vuln-code-snippet end php_extract_list_destruct
