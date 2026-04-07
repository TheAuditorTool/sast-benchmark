<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00457(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('callback');
    $args = explode(',', $req->param('args'));
    $result = call_user_func_array($func, $args);
    return BenchmarkResponse::ok((string)$result);
}
