<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00330(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $safe = htmlspecialchars($input, ENT_QUOTES);
    echo "Hello: $safe";
    return BenchmarkResponse::ok($safe);
}
