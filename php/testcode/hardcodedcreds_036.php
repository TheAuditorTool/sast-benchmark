<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_k8s_secret_mount
function hardcodedcreds036(BenchmarkRequest $req): BenchmarkResponse {
    $pass = trim(file_get_contents('/var/run/secrets/db_pass')); // vuln-code-snippet safe-line php_hardcoded_k8s_secret_mount
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_k8s_secret_mount
