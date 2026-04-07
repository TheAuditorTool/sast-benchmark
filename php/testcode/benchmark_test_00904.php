<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00904(BenchmarkRequest $req): BenchmarkResponse {
    $keys = ['name', 'email', 'phone'];
    $values = [
        $req->post('name'),
        $req->post('email'),
        $req->post('phone'),
    ];
    $data = array_combine($keys, $values);
    return BenchmarkResponse::json($data);
}
