<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00552(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $name = array_key_exists('name', $data) ? $data['name'] : 'default';
    $email = array_key_exists('email', $data) ? $data['email'] : 'none';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
