<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_vault_read
function hardcodedcreds030(BenchmarkRequest $req): BenchmarkResponse {
    $vaultAddr = getenv('VAULT_ADDR');
    $vaultToken = getenv('VAULT_TOKEN');
    $response = file_get_contents($vaultAddr . '/v1/secret/data/db', false, stream_context_create([
        'http' => ['header' => 'X-Vault-Token: ' . $vaultToken],
    ]));
    $password = json_decode($response, true)['data']['data']['password']; // vuln-code-snippet safe-line php_hardcoded_vault_read
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $password);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_vault_read
