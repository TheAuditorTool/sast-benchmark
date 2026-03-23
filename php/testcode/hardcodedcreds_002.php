<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_db_env
function hardcodedcreds002(BenchmarkRequest $req): BenchmarkResponse {
    $dsn = 'mysql:host=localhost;dbname=app';
    $pdo = new PDO($dsn, getenv('DB_USER'), getenv('DB_PASS')); // vuln-code-snippet safe-line php_hardcoded_db_env
    $rows = $pdo->query("SELECT id, name FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_hardcoded_db_env
