<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00334(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl');
    $client = new SoapClient($wsdl);
    $result = $client->__getFunctions();
    return BenchmarkResponse::json($result);
}
