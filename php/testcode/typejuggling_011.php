<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_json_type_confusion
function typejuggling011(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), false);
    $expectedId = 0;
    if ($body->user_id == $expectedId) { // vuln-code-snippet vuln-line php_tj_json_type_confusion
        return BenchmarkResponse::ok('Admin access granted');
    }
    return BenchmarkResponse::ok('Regular user');
}
// vuln-code-snippet end php_tj_json_type_confusion
