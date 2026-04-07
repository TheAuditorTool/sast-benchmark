<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01212(BenchmarkRequest $req): BenchmarkResponse {
    $returnTo = $req->post('return_to');
    if (strpos($returnTo, '/') !== 0 || strpos($returnTo, '//') === 0) {
        return BenchmarkResponse::redirect('/dashboard');
    }
    return BenchmarkResponse::redirect($returnTo);
}
