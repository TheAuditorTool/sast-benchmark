<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00880(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'guest';
    extract($req->queryParams);
    return BenchmarkResponse::ok("role: $role");
}
