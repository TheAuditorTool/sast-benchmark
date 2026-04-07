<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00913(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $table = $req->param('table');
    $pdo->exec("DROP TABLE IF EXISTS " . $table);
    return BenchmarkResponse::ok("Table dropped");
}
