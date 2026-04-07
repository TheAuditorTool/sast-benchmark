<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00565(BenchmarkRequest $req): BenchmarkResponse {
    $var = $req->param('var');
    $val = $req->param('val');
    $$var = $val;
    return BenchmarkResponse::ok('assigned');
}
