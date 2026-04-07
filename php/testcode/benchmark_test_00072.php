<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00072(BenchmarkRequest $req): BenchmarkResponse {
    $f       = $req->param('f');
    $viewsDir = realpath(__DIR__ . '/views') . DIRECTORY_SEPARATOR;
    $real    = realpath($viewsDir . $f);
    if ($real === false || strpos($real, $viewsDir) !== 0) {
        return BenchmarkResponse::badRequest('Forbidden path');
    }
    include $real;
    return BenchmarkResponse::ok('Rendered');
}
