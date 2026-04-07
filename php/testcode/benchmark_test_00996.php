<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00996(BenchmarkRequest $req): BenchmarkResponse {
    $data = array_merge($_POST, ['created_at' => date('Y-m-d')]);
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('created');
}
