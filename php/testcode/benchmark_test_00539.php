<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00539(BenchmarkRequest $req): BenchmarkResponse {
    $page = trim($req->param('page'));
    include $page;
    return BenchmarkResponse::ok('Done');
}
