<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00363(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $email = $req->param('email');
    $data = compact('name', 'email');
    return BenchmarkResponse::json($data);
}
