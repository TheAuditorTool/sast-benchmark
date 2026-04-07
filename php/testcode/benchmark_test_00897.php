<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00897(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('class');
    if (class_exists($input)) {
        $obj = new $input();
    }
    return BenchmarkResponse::ok('done');
}
