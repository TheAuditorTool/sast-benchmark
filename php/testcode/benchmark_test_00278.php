<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00278(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $attrs = $_POST;
    foreach ($attrs as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('updated');
}
