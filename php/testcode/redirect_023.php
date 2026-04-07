<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_protocol_relative_host
function redirect023(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $path = $req->param('path');
    header('Location: //' . $host . $path); // vuln-code-snippet vuln-line php_redirect_protocol_relative_host
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_protocol_relative_host
