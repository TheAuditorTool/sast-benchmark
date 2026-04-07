<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_css_style_injection
function xss028(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('color');
    $html = "<div style=\"color: $input\">Hello</div>"; // vuln-code-snippet vuln-line php_xss_css_style_injection
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_css_style_injection
