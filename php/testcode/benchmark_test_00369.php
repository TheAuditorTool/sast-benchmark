<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00369(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $action($req->param('data'));
    return BenchmarkResponse::ok('executed');
}
