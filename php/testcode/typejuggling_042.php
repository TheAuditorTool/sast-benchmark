<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_json_type_check_int
function typejuggling042(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true, 512, JSON_THROW_ON_ERROR);
    if (!is_int($data['id'])) { // vuln-code-snippet safe-line php_tj_json_type_check_int
        return BenchmarkResponse::badRequest('invalid');
    }
    if ($data['id'] === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
// vuln-code-snippet end php_tj_json_type_check_int
