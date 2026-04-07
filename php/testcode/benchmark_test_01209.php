<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01209(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('next');
    return BenchmarkResponse::redirect($url);
}
