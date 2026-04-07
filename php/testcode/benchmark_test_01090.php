<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01090(BenchmarkRequest $req): BenchmarkResponse {
    $needle = $req->param('val');
    $arr = ['admin', 'user', 'guest'];
    $pos = array_search($needle, $arr);
    if ($pos !== false) {
        return BenchmarkResponse::ok('found:' . $pos);
    }
    return BenchmarkResponse::badRequest('not found');
}
