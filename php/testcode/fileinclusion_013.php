<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_wrapper_data
function fileinclusion013(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->param('payload');
    include("data://text/plain;base64," . $payload); // vuln-code-snippet vuln-line php_fi_wrapper_data
    return BenchmarkResponse::ok("data loaded");
}
// vuln-code-snippet end php_fi_wrapper_data
