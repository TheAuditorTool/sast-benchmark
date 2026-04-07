<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00002(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $path = $req->param('path');
    header('Location: //' . $host . $path);
    return BenchmarkResponse::ok('');
}
