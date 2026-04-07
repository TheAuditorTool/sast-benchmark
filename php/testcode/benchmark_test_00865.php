<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00865(BenchmarkRequest $req): BenchmarkResponse {
    $vaultAddr = getenv('VAULT_ADDR');
    $vaultToken = getenv('VAULT_TOKEN');
    $response = file_get_contents($vaultAddr . '/v1/secret/data/db', false, stream_context_create([
        'http' => ['header' => 'X-Vault-Token: ' . $vaultToken],
    ]));
    $password = json_decode($response, true)['data']['data']['password'];
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $password);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
