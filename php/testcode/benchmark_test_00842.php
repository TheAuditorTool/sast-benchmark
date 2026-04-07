<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00842(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->cookie('name');
    $$name = $req->cookie('val');
    return BenchmarkResponse::ok('assigned');
}
