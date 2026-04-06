<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_wp_redirect
function wp_redirect_unsafe(string $location): void {
    header("Location: " . $location);
}

function redirect011(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('redirect_to');
    wp_redirect_unsafe($url); // vuln-code-snippet vuln-line php_redirect_wp_redirect
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_wp_redirect
