<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00293(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE settings SET value = ? WHERE key = 'theme'")->execute([$req->post('theme')]);
    return BenchmarkResponse::ok('Theme updated');
}
