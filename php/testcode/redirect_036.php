<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_wp_safe_allowed_hosts
function redirect036(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    wp_safe_redirect($url, 302, 'my-plugin'); // vuln-code-snippet safe-line php_redirect_wp_safe_allowed_hosts
    exit;
}
// vuln-code-snippet end php_redirect_wp_safe_allowed_hosts
