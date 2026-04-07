<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_json_int_string_loose
function typejuggling024(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr());
    if ($data->id == "admin") { // vuln-code-snippet vuln-line php_tj_json_int_string_loose // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('access');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_json_int_string_loose
