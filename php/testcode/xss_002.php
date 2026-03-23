<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_htmlspecialchars
function xss_htmlspecialchars(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');

    $safe = htmlspecialchars($name, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_htmlspecialchars
    $html = '<html><body><h1>Hello, ' . $safe . '!</h1></body></html>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_htmlspecialchars
