<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01028(BenchmarkRequest $req): BenchmarkResponse {
    $args = json_decode($req->bodyStr(), true)['args'];
    $model = new stdClass();
    foreach ($args as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('resolved');
}
