<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00302(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    $name = $data['name'] ?? '';
    $email = $data['email'] ?? '';
    $bio = $data['bio'] ?? '';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email, 'bio' => $bio]);
}
