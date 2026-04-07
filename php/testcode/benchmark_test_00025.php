<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00025(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->post('code');
    $encoded = base64_encode($code);
    ob_start();
    include("data://text/plain;base64," . $encoded);
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
