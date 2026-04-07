<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00455(BenchmarkRequest $req): BenchmarkResponse {
    $msg = $req->param('msg');
    header('HTTP/1.1 200 ' . $msg);
    return BenchmarkResponse::ok($msg);
}
