<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_content_type
function headerinj009(BenchmarkRequest $req): BenchmarkResponse {
    $format = $req->param('format');
    header("Content-Type: " . $format); // vuln-code-snippet vuln-line php_headerinj_content_type
    return BenchmarkResponse::ok('{"status":"ok"}');
}
// vuln-code-snippet end php_headerinj_content_type
