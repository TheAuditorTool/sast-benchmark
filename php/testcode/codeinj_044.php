<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_json_decode_only
function codeinj044(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true); // vuln-code-snippet safe-line php_codeinj_json_decode_only
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('invalid JSON');
    }
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_codeinj_json_decode_only
