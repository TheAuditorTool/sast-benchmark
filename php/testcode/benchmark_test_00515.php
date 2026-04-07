<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00515(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new stdClass();
    $data = $_POST;
    foreach ($data as $k => $v) {
        $dto->$k = $v;
    }
    return BenchmarkResponse::ok('hydrated');
}
