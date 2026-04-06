<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_array_map_callback
function unsafereflect011(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $data = explode(',', $req->param('data'));
    $result = array_map($fn, $data); // vuln-code-snippet vuln-line php_reflect_array_map_callback
    return BenchmarkResponse::json($result);
}
// vuln-code-snippet end php_reflect_array_map_callback
