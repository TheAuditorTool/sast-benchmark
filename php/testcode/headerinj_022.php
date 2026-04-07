<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_cors_origin_reflect
function headerinj022(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->param('origin');
    header('Access-Control-Allow-Origin: ' . $origin); // vuln-code-snippet vuln-line php_headerinj_cors_origin_reflect
    header('Access-Control-Allow-Credentials: true');
    return BenchmarkResponse::ok('cors set');
}
// vuln-code-snippet end php_headerinj_cors_origin_reflect
