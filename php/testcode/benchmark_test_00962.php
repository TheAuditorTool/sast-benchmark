<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00962(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = (string) substr($_POST['name'] ?? '', 0, 255);
    $model->age = (int) ($_POST['age'] ?? 0);
    return BenchmarkResponse::ok('cast');
}
