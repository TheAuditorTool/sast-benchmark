<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_apikey_in_cookie
function csrf027(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = $req->cookie('api_key');
    if (empty($apiKey) || !validateApiKey($apiKey)) {
        return BenchmarkResponse::error('unauthorized');
    }
    $result = performApiAction($req->param('action')); // vuln-code-snippet vuln-line php_csrf_apikey_in_cookie
    return BenchmarkResponse::json(['result' => $result]);
}
// vuln-code-snippet end php_csrf_apikey_in_cookie
