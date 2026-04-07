<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_soap_wsdl_entity
function xxe026(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl');
    $client = new SoapClient($wsdl); // vuln-code-snippet vuln-line php_xxe_soap_wsdl_entity
    return BenchmarkResponse::ok('client created');
}
// vuln-code-snippet end php_xxe_soap_wsdl_entity
