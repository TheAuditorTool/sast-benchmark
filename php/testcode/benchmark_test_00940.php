<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00940(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $data = $req->post('data');
    file_put_contents($userPath, $data);
    return BenchmarkResponse::ok('written');
}
