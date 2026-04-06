<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_json_throw
function deserial016(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true, 512, JSON_THROW_ON_ERROR); // vuln-code-snippet safe-line php_deser_json_throw
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_json_throw
