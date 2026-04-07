<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00859(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    if (is_numeric($val)) {
        $num = intval($val, 0);
        return BenchmarkResponse::ok((string)$num);
    }
    return BenchmarkResponse::badRequest('not numeric');
}
