<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00003(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    foreach ($_POST as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('saved');
}
