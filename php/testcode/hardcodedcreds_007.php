<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_define
define('DB_PASSWORD', 'super_secret_p@ss'); // vuln-code-snippet vuln-line php_hardcoded_define

function hardcodedcreds007(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', DB_PASSWORD);
    return BenchmarkResponse::ok('Connected');
}
// vuln-code-snippet end php_hardcoded_define
