<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00999(BenchmarkRequest $req): BenchmarkResponse {
    $enc = $req->param('enc');
    header('Transfer-Encoding: ' . $enc);
    return BenchmarkResponse::ok('encoding set');
}
