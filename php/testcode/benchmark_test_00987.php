<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00987(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $url = $req->param('url') . '?' . SID;
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
