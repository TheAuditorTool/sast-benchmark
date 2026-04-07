<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00889(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name'] ?? '';
    $model->email = $_POST['email'] ?? '';
    return BenchmarkResponse::ok('assigned');
}
