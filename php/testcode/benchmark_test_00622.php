<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00622(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $port = (int)$req->param('port');
    $fp = fsockopen($host, $port);
    $response = fread($fp, 4096);
    fclose($fp);
    return BenchmarkResponse::ok($response);
}
