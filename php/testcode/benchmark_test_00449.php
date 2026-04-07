<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00449(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr());
    if ($data->id == "admin") {
        return BenchmarkResponse::ok('access');
    }
    return BenchmarkResponse::badRequest('denied');
}
