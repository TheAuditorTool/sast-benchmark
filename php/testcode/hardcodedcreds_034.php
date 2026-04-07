<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_docker_secret
function hardcodedcreds034(BenchmarkRequest $req): BenchmarkResponse {
    $pass = trim(file_get_contents('/run/secrets/db_password')); // vuln-code-snippet safe-line php_hardcoded_docker_secret
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_docker_secret
