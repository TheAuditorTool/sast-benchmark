<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_regex_newline_bypass
function redirect028(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (preg_match('/^https?:\/\/example\.com/', $url)) {
        header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_regex_newline_bypass
    }
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_regex_newline_bypass
