<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00433(BenchmarkRequest $req): BenchmarkResponse {
    $ssm = new stdClass();
    $result = $ssm->getParameter(['Name' => '/app/db-pass', 'WithDecryption' => true]);
    $pass = $result['Parameter']['Value'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
