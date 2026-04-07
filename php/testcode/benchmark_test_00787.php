<?php
require_once __DIR__ . '/shared.php';

/** @phpstan-immutable */
class StrictModel {
    public string $name = '';
}

function benchmarkTest00787(BenchmarkRequest $req): BenchmarkResponse {
    $model = new StrictModel();
    $model->name = $req->param('name');
    return BenchmarkResponse::ok($model->name);
}
