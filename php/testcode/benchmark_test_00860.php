<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00860(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['lifetime' => 0, 'path' => '/']);
    session_start();
    return BenchmarkResponse::ok('session started');
}
