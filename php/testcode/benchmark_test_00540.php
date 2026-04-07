<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00540(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $clean = str_replace(["\r", "\n", "\0"], '', $input);
    header("X-Data: " . $clean);
    return BenchmarkResponse::ok('Done');
}
