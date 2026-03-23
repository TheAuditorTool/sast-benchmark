<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_soap
function xxe007(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl');
    $client = new SoapClient($wsdl); // vuln-code-snippet vuln-line php_xxe_soap
    $result = $client->__getFunctions();
    return BenchmarkResponse::json($result);
}
// vuln-code-snippet end php_xxe_soap
