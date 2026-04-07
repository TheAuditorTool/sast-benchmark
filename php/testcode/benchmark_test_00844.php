<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00844(BenchmarkRequest $req): BenchmarkResponse {
    $client = new Aws\Kms\KmsClient(['region' => 'us-east-1', 'version' => 'latest']);
    $encryptedBlob = file_get_contents(__DIR__ . '/../config/db_pass.enc');
    $result = $client->decrypt(['CiphertextBlob' => $encryptedBlob]);
    $password = $result['Plaintext'];
    return BenchmarkResponse::ok('Decrypted secret via KMS');
}
