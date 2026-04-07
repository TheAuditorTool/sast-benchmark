<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00794(BenchmarkRequest $req): BenchmarkResponse {
    session_name('MYAPP_SID');
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Logged in');
}
