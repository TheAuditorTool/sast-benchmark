<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_secrets_manager
function hardcodedcreds032(BenchmarkRequest $req): BenchmarkResponse {
    $sm = new stdClass();
    $result = $sm->getSecretValue(['SecretId' => 'prod/db']); // vuln-code-snippet safe-line php_hardcoded_secrets_manager
    $pass = json_decode($result['SecretString'], true)['password'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_secrets_manager
