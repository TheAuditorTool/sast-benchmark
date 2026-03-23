<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_multicontext
function xss_multicontext(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('search');

    $html = '<input type="text" value="' . $input . '">' // vuln-code-snippet vuln-line php_xss_multicontext
          . '<p>Results for: ' . $input . '</p>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_multicontext
