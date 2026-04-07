<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00617(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email', 'bio'];
    $data = array_intersect_key($_POST, array_flip($allowed));
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('saved');
}
