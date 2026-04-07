<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00001(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $schema = ['name' => 'string', 'email' => 'string'];
    foreach ($data as $key => $val) {
        if (!isset($schema[$key])) {
            return BenchmarkResponse::badRequest("Unknown field: $key");
        }
    }
    $pdo = getDbConnection();
    $pdo->prepare("INSERT INTO users (name, email) VALUES (?, ?)")
        ->execute([$data['name'] ?? '', $data['email'] ?? '']);
    return BenchmarkResponse::ok('User created');
}
