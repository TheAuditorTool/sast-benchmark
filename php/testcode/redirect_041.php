<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_relative_only_enforced
function redirect041(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (strpos($url, 'http') === 0) {
        return BenchmarkResponse::badRequest('no absolute');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_relative_only_enforced
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_relative_only_enforced
