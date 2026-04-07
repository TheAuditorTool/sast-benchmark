<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01023(BenchmarkRequest $req): BenchmarkResponse {
    $vaultAddr = getenv('VAULT_ADDR');
    $vaultToken = getenv('VAULT_TOKEN');
    $ch = curl_init($vaultAddr . '/v1/secret/data/db');
    curl_setopt($ch, CURLOPT_HTTPHEADER, ["X-Vault-Token: $vaultToken"]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    curl_close($ch);
    $secret = json_decode($response, true);
    $password = $secret['data']['data']['password'];
    return BenchmarkResponse::ok('Retrieved secret from Vault');
}
