<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00581(BenchmarkRequest $req): BenchmarkResponse {
    $authenticated = false;
    $user_role = 'anonymous';
    extract($req->cookies);
    if ($authenticated) {
        return BenchmarkResponse::ok('Welcome, ' . $user_role);
    }
    return BenchmarkResponse::ok('Please log in');
}
