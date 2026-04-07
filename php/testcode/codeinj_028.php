<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_include_data_stream
function codeinj028(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    $stream = 'data://text/plain;base64,' . base64_encode($code);
    ob_start();
    include $stream; // vuln-code-snippet vuln-line php_codeinj_include_data_stream
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_codeinj_include_data_stream
