<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01120(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    switch ($role) {
        case 0:
            return BenchmarkResponse::ok('admin access granted');
        case 'editor':
            return BenchmarkResponse::ok('editor access');
        default:
            return BenchmarkResponse::ok('guest access');
    }
}
