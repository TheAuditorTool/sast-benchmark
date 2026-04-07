<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_htmlentities_html5
function xss033(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    $safe = htmlentities($input, ENT_QUOTES | ENT_HTML5, 'UTF-8'); // vuln-code-snippet safe-line php_xss_htmlentities_html5
    return BenchmarkResponse::html("<span>$safe</span>");
}
// vuln-code-snippet end php_xss_htmlentities_html5
