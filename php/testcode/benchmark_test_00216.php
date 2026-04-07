<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00216(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['name' => $_POST['name'], 'email' => $_POST['email']];
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('mapped');
}
