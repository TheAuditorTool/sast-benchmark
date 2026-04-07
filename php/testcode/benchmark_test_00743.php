<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00743(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new SoapClient(null, ['location' => $url, 'uri' => 'http://example.com']);
    return BenchmarkResponse::ok('soap client created');
}
