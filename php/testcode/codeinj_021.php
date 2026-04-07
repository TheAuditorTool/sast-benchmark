<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_cookie_callback
function codeinj021(BenchmarkRequest $req): BenchmarkResponse {
    $handler = $req->cookie('handler');
    $data = $req->param('data');
    $result = call_user_func($handler, $data); // vuln-code-snippet vuln-line php_codeinj_cookie_callback
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_cookie_callback
