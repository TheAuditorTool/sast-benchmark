<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00754(BenchmarkRequest $req): BenchmarkResponse {
    $user = $req->param('user');
    if ($user == true) {
        return BenchmarkResponse::ok('logged in');
    }
    return BenchmarkResponse::badRequest('denied');
}
