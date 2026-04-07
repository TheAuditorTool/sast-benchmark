<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00626(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name'];
    return BenchmarkResponse::ok('updated');
}
