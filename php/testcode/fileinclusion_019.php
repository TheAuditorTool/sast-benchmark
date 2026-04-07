<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_data_base64_stream
function fileinclusion019(BenchmarkRequest $req): BenchmarkResponse {
    $b64 = $req->param('b64');
    include 'data://text/plain;base64,' . $b64; // vuln-code-snippet vuln-line php_fi_data_base64_stream
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_data_base64_stream
