<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_ob_include_unescaped
function xss030(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('msg');
    ob_start();
    echo $input; // vuln-code-snippet vuln-line php_xss_ob_include_unescaped
    $html = ob_get_clean();
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_ob_include_unescaped
