<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_define_dbpass
function hardcodedcreds015(BenchmarkRequest $req): BenchmarkResponse {
    define('DB_PASSWORD', 'hunter2'); // vuln-code-snippet vuln-line php_hardcoded_define_dbpass
    $pdo = new PDO(
        'mysql:host=localhost;dbname=app',
        'root',
        DB_PASSWORD
    );
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_define_dbpass
