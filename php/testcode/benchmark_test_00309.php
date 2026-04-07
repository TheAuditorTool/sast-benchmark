<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00309(BenchmarkRequest $req): BenchmarkResponse {
    $name  = $_POST['name']  ?? 'anonymous';
    $email = $_POST['email'] ?? '';
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
