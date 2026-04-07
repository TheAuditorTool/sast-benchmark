<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00295(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    if (in_array($role, [1, 2, 3])) {
        return BenchmarkResponse::ok('admin');
    }
    return BenchmarkResponse::badRequest('denied');
}
