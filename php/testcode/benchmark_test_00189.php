<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00189(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    if ($req->param('action') === 'elevate') {
        session_regenerate_id(true);
        $_SESSION['role'] = 'admin';
    }
    return BenchmarkResponse::ok('elevated');
}
