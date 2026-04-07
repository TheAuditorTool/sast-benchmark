<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00775(BenchmarkRequest $req): BenchmarkResponse {
    $referer = $req->header('Referer');
    if (empty($referer)) {
        return BenchmarkResponse::badRequest('Missing referer');
    }
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE users SET email = ? WHERE id = ?")->execute([$req->post('email'), $req->post('id')]);
    return BenchmarkResponse::ok('Email updated');
}
