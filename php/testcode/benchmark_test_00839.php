<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00839(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    header('Content-Disposition: attachment; filename="' . $name . '"');
    return BenchmarkResponse::ok('download ready');
}
