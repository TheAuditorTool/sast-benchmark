<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_config_array
function hardcodedcreds008(BenchmarkRequest $req): BenchmarkResponse {
    $config = [
        'db_host' => 'localhost',
        'db_user' => 'admin',
        'db_pass' => 'ProductionP@ss2026!', // vuln-code-snippet vuln-line php_hardcoded_config_array
        'db_name' => 'myapp',
    ];
    $pdo = new PDO(
        "mysql:host={$config['db_host']};dbname={$config['db_name']}",
        $config['db_user'],
        $config['db_pass']
    );
    return BenchmarkResponse::ok('Connected');
}
// vuln-code-snippet end php_hardcoded_config_array
