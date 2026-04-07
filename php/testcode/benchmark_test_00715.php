<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00715(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $_SESSION['user'] = $req->param('user');
    return BenchmarkResponse::ok('session started');
}
