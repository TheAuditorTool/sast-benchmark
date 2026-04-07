<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_json_only_api
function xxe034(BenchmarkRequest $req): BenchmarkResponse {
    $ct = $req->header('Content-Type');
    if (strpos($ct, 'application/json') === false) {
        return BenchmarkResponse::badRequest('json only');
    }
    $data = json_decode($req->bodyStr(), true); // vuln-code-snippet safe-line php_xxe_json_only_api
    return BenchmarkResponse::ok(json_encode($data));
}
// vuln-code-snippet end php_xxe_json_only_api
