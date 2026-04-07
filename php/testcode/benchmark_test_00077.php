<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00077(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $decoded = json_decode($data, true);
    $model = new stdClass();
    foreach ($decoded as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('filled');
}
