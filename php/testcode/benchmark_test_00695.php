<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00695(BenchmarkRequest $req): BenchmarkResponse {
    session_id(md5((string) time()));
    session_start();
    return BenchmarkResponse::ok('session started');
}
