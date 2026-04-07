<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00365(BenchmarkRequest $req): BenchmarkResponse {
    $age = $req->param('age');
    $filtered = filter_var($age, FILTER_VALIDATE_INT, ['options' => ['min_range' => 0, 'max_range' => 150]]);
    if ($filtered === false) {
        return BenchmarkResponse::badRequest('Invalid age');
    }
    return BenchmarkResponse::json(['age' => $filtered]);
}
