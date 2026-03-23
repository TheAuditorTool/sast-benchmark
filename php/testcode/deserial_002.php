<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_json_decode
function deserial_json_decode(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->bodyStr();
    $data = json_decode($input, true); // vuln-code-snippet safe-line php_deser_json_decode
    return BenchmarkResponse::json(['name' => $data['name'] ?? 'unknown']);
}
// vuln-code-snippet end php_deser_json_decode
