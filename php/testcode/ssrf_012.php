<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_soap_wsdl
function ssrf012(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl_url');
    $client = new SoapClient($wsdl); // vuln-code-snippet vuln-line php_ssrf_soap_wsdl
    $result = $client->__getFunctions();
    return BenchmarkResponse::json($result);
}
// vuln-code-snippet end php_ssrf_soap_wsdl
