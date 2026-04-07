<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00718(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    header("X-User-Data: " . rawurlencode($input));
    return BenchmarkResponse::ok('Done');
}
