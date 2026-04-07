<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_soap_location
function ssrf026(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new SoapClient(null, ['location' => $url, 'uri' => 'http://example.com']); // vuln-code-snippet vuln-line php_ssrf_soap_location
    return BenchmarkResponse::ok('soap client created');
}
// vuln-code-snippet end php_ssrf_soap_location
