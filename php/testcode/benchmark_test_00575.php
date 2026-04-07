<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00575(BenchmarkRequest $req): BenchmarkResponse {
    $sm = new stdClass();
    $result = $sm->getSecretValue(['SecretId' => 'prod/db']);
    $pass = json_decode($result['SecretString'], true)['password'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
