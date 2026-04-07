<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01210(BenchmarkRequest $req): BenchmarkResponse {
    $returnTo = $req->post('return_to');
    return BenchmarkResponse::redirect($returnTo);
}
