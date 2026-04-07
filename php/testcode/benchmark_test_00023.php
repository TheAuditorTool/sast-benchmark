<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00023(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    header('Link: <' . $url . '>; rel=preload');
    return BenchmarkResponse::ok('preload hint set');
}
