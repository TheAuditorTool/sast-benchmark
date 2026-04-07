<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00677(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('hash');
    if (md5($input) == '0e462097431906509019562988736854') {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}
