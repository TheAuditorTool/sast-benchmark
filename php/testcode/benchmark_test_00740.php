<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00740(BenchmarkRequest $req): BenchmarkResponse {
    $count = $req->param('count');
    header("X-Result-Count: " . (int)$count);
    return BenchmarkResponse::ok('Results returned');
}
