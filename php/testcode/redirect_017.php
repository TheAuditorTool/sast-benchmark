<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_wp_unsafe
function redirect017(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    wp_redirect($url); // vuln-code-snippet vuln-line php_redirect_wp_unsafe
    exit;
}
// vuln-code-snippet end php_redirect_wp_unsafe
