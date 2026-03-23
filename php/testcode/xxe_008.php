<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_json_input
function xxe008(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true); // vuln-code-snippet safe-line php_xxe_json_input
    if ($data === null) {
        return BenchmarkResponse::badRequest('invalid json');
    }
    $name = $data['name'] ?? 'unknown';
    return BenchmarkResponse::ok("hello $name");
}
// vuln-code-snippet end php_xxe_json_input
