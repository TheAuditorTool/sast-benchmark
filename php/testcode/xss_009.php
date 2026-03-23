<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_json_in_script
function xss_json_in_script(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('config');

    $html = '<script>var x = ' . $input . ';</script>'; // vuln-code-snippet vuln-line php_xss_json_in_script

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_json_in_script
