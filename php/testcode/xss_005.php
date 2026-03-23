<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_sprintf_html
function xss_sprintf_html(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('title');

    $html = sprintf('<div class="title">%s</div>', $input); // vuln-code-snippet vuln-line php_xss_sprintf_html

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_sprintf_html
