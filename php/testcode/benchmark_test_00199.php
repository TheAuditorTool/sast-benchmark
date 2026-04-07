<?php
require_once __DIR__ . '/shared.php';

define('DB_PASSWORD', 'super_secret_p@ss');

function benchmarkTest00199(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', DB_PASSWORD);
    return BenchmarkResponse::ok('Connected');
}
