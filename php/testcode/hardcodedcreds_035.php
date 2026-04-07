<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_ini_outside_webroot
function hardcodedcreds035(BenchmarkRequest $req): BenchmarkResponse {
    $secrets = parse_ini_file('/etc/app/secrets.ini'); // vuln-code-snippet safe-line php_hardcoded_ini_outside_webroot
    $pass = $secrets['db_password'] ?? '';
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_ini_outside_webroot
