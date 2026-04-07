<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00496(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $postData = $_POST;
    array_walk($postData, function ($v, $k) use ($model) {
        $model->$k = $v;
    });
    return BenchmarkResponse::ok('stored');
}
