<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_include_data
function codeinj014(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->post('code');
    $encoded = base64_encode($code);
    ob_start();
    include("data://text/plain;base64," . $encoded); // vuln-code-snippet vuln-line php_codeinj_include_data
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_codeinj_include_data
