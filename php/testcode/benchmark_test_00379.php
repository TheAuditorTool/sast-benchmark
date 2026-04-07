<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00379(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int) $req->param('id');
    require_once __DIR__ . '/views/' . $id . '.php';
    return BenchmarkResponse::ok('Rendered');
}
