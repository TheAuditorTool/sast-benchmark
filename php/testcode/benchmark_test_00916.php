<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00916(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    header("Content-Security-Policy: default-src 'self'");
    $safe = htmlspecialchars($input, ENT_QUOTES);
    echo "<h1>$safe</h1>";
    return BenchmarkResponse::ok($safe);
}
