<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_base64_cred
function hardcodedcreds024(BenchmarkRequest $req): BenchmarkResponse {
    $dbPass = base64_decode('aHVudGVyMg=='); // vuln-code-snippet vuln-line php_hardcoded_base64_cred
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', $dbPass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_base64_cred
