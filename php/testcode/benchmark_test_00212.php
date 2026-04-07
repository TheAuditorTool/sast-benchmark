<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00212(BenchmarkRequest $req): BenchmarkResponse {
    $config = [
        'db_host' => 'localhost',
        'db_user' => 'admin',
        'db_pass' => 'ProductionP@ss2026!',
        'db_name' => 'myapp',
    ];
    $pdo = new PDO(
        "mysql:host={$config['db_host']};dbname={$config['db_name']}",
        $config['db_user'],
        $config['db_pass']
    );
    return BenchmarkResponse::ok('Connected');
}
