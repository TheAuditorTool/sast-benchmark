<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00730(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $msg = new \Google\Protobuf\Internal\Message();
    $msg->mergeFromString($body);
    return BenchmarkResponse::ok('parsed');
}
