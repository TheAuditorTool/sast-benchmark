<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_db_password
function hardcodedcreds001(BenchmarkRequest $req): BenchmarkResponse {
    $dsn = 'mysql:host=localhost;dbname=app';
    $pdo = new PDO($dsn, "root", "s3cr3t_p4ss"); // vuln-code-snippet vuln-line php_hardcoded_db_password
    $rows = $pdo->query("SELECT id, name FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_hardcoded_db_password
