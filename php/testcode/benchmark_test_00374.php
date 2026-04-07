<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00374(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $port = (int)$req->param('port');
    $fp = fsockopen($host, $port, $errno, $errstr, 5);
    if ($fp) {
        fwrite($fp, "GET / HTTP/1.0\r\nHost: $host\r\n\r\n");
        $response = stream_get_contents($fp);
        fclose($fp);
        return BenchmarkResponse::ok($response);
    }
    return BenchmarkResponse::error("Connection failed: $errstr");
}
