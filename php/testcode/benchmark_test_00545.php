<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00545(BenchmarkRequest $req): BenchmarkResponse {
    $data = $_POST;
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    $db = getDbConnection();
    $db->exec("INSERT INTO users DEFAULT VALUES");
    return BenchmarkResponse::ok('created');
}
