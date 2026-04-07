<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01213(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $data = file_get_contents($url);
    return BenchmarkResponse::ok($data);
}
