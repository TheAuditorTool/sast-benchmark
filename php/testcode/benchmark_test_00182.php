<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00182(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('host');
    if (!preg_match('/^[a-zA-Z0-9.]+$/', $input)) {
        return BenchmarkResponse::badRequest("invalid host");
    }
    $output = [];
    exec("ping -c 3 " . $input, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
