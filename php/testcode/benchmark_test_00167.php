<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00167(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $pdo->prepare("UPDATE users SET name = ?, email = ? WHERE id = ?")
        ->execute([$req->post('name'), $req->post('email'), $id]);
    return BenchmarkResponse::ok('User updated');
}
