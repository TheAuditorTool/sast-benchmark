<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_env_no_default
function hardcodedcreds033(BenchmarkRequest $req): BenchmarkResponse {
    $pass = getenv('DB_PASSWORD') ?: throw new RuntimeException('DB_PASSWORD not set'); // vuln-code-snippet safe-line php_hardcoded_env_no_default
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_env_no_default
