<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00047(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $event = unserialize($body);
    processWebhookEvent($event);
    return BenchmarkResponse::ok('received');
}
