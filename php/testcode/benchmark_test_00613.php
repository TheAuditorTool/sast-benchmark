<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00613(BenchmarkRequest $req): BenchmarkResponse {
    extract($_POST);
    $data = compact(...array_keys($_POST));
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('compacted');
}
