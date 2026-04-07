<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00477(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), true);
    $model = new stdClass();
    foreach ($body as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('updated');
}
