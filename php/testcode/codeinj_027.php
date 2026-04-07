<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_include_php_input
function codeinj027(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    ob_start();
    include 'php://input'; // vuln-code-snippet vuln-line php_codeinj_include_php_input
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_codeinj_include_php_input
