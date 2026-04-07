<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_ssm_getparam
function hardcodedcreds031(BenchmarkRequest $req): BenchmarkResponse {
    $ssm = new stdClass();
    $result = $ssm->getParameter(['Name' => '/app/db-pass', 'WithDecryption' => true]); // vuln-code-snippet safe-line php_hardcoded_ssm_getparam
    $pass = $result['Parameter']['Value'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_ssm_getparam
