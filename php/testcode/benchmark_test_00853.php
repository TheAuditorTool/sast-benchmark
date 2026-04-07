<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00853(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl_url');
    $client = new SoapClient($wsdl);
    $result = $client->__getFunctions();
    return BenchmarkResponse::json($result);
}
