<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00605(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('token');
    if (!preg_match('/^[a-zA-Z0-9_-]+$/', $input)) {
        return BenchmarkResponse::badRequest('Invalid token format');
    }
    header("X-Auth-Token: " . $input);
    return BenchmarkResponse::ok('Authenticated');
}
