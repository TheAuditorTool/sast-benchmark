<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00823(BenchmarkRequest $req): BenchmarkResponse {
    $filtered = array_map(fn($v) => $v, $_POST);
    $model = new stdClass();
    foreach ($filtered as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('mapped');
}
