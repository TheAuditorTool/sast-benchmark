<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_dsn_password
function hardcodedcreds016(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', 's3cr3t'); // vuln-code-snippet vuln-line php_hardcoded_dsn_password
    $stmt = $pdo->query('SELECT COUNT(*) FROM users');
    $count = $stmt->fetchColumn();
    return BenchmarkResponse::ok((string)$count);
}
// vuln-code-snippet end php_hardcoded_dsn_password
