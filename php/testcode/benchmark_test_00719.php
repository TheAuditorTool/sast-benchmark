<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00719(BenchmarkRequest $req): BenchmarkResponse {
    $displayVars = ['color' => $req->param('color')];
    extract($displayVars);
    echo $color;
    return BenchmarkResponse::ok('rendered');
}
