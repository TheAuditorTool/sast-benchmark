<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00206(BenchmarkRequest $req): BenchmarkResponse {
    $handler = $req->cookie('handler');
    $data = $req->param('data');
    $result = call_user_func($handler, $data);
    return BenchmarkResponse::ok((string) $result);
}
