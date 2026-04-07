<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01102(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $allowed = ['name' => true, 'email' => true, 'phone' => true];
    $safe = array_intersect_key($data, $allowed);
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE users SET name = ?, email = ?, phone = ? WHERE id = ?")
        ->execute([$safe['name'] ?? '', $safe['email'] ?? '', $safe['phone'] ?? '', $req->param('id')]);
    return BenchmarkResponse::ok('User updated');
}
