<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00098(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    switch ($type) {
        case 1:
            return BenchmarkResponse::ok('admin');
        case 2:
            return BenchmarkResponse::ok('user');
    }
    return BenchmarkResponse::badRequest('denied');
}
