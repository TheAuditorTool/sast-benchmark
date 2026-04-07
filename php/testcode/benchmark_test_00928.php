<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00928(BenchmarkRequest $req): BenchmarkResponse {
    $wsdl = $req->param('wsdl');
    $client = new SoapClient($wsdl);
    return BenchmarkResponse::ok('client created');
}
