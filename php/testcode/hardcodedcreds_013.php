<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_dotenv
function hardcodedcreds013(BenchmarkRequest $req): BenchmarkResponse {
    $dotenv = Dotenv\Dotenv::createImmutable(__DIR__ . '/..');
    $dotenv->load();
    $dbPass = $_ENV['DB_PASSWORD']; // vuln-code-snippet safe-line php_hardcoded_dotenv
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', $dbPass);
    return BenchmarkResponse::ok('Connected via dotenv');
}
// vuln-code-snippet end php_hardcoded_dotenv
