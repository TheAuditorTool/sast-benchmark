<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00573(BenchmarkRequest $req): BenchmarkResponse {
    $dotenv = Dotenv\Dotenv::createImmutable(__DIR__ . '/..');
    $dotenv->load();
    $dbPass = $_ENV['DB_PASSWORD'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', $dbPass);
    return BenchmarkResponse::ok('Connected via dotenv');
}
