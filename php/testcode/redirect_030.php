<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_json_redirect_field
function redirect030(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $url = $data['redirect'];
    header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_json_redirect_field
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_json_redirect_field
