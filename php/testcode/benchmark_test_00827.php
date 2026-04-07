<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00827(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('input');
    $result = (function() use ($data) {
        $local = $data;
        return $local;
    })();
    return BenchmarkResponse::ok($result);
}
