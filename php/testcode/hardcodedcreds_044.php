<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_runtime_loaded_config
function hardcodedcreds044(BenchmarkRequest $req): BenchmarkResponse {
    $config = json_decode(file_get_contents(getenv('CONFIG_PATH')), true); // vuln-code-snippet safe-line php_hardcoded_runtime_loaded_config
    $pass = $config['db_password'] ?? '';
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_runtime_loaded_config
