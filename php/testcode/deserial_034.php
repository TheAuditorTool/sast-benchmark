<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_json_arrays_only
function deserial034(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true); // vuln-code-snippet safe-line php_deser_json_arrays_only
    if (!is_array($data)) {
        return BenchmarkResponse::badRequest('expected JSON array/object');
    }
    return BenchmarkResponse::json(['count' => count($data)]);
}
// vuln-code-snippet end php_deser_json_arrays_only
