<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00412(BenchmarkRequest $req): BenchmarkResponse {
    $domain = $req->param('domain');
    $result = `nslookup $domain`;
    return BenchmarkResponse::ok($result ?? "");
}
