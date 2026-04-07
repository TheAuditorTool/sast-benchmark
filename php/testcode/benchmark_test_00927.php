<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00927(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->param('payload');
    include("data://text/plain;base64," . $payload);
    return BenchmarkResponse::ok("data loaded");
}
