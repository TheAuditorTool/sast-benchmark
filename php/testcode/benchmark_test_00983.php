<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00983(BenchmarkRequest $req): BenchmarkResponse {
    $input = array_merge($_GET, $_POST);
    $model = new stdClass();
    foreach ($input as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('updated');
}
