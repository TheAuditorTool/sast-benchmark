<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_kms_decrypt
function hardcodedcreds014(BenchmarkRequest $req): BenchmarkResponse {
    $client = new Aws\Kms\KmsClient(['region' => 'us-east-1', 'version' => 'latest']);
    $encryptedBlob = file_get_contents(__DIR__ . '/../config/db_pass.enc');
    $result = $client->decrypt(['CiphertextBlob' => $encryptedBlob]);
    $password = $result['Plaintext']; // vuln-code-snippet safe-line php_hardcoded_kms_decrypt
    return BenchmarkResponse::ok('Decrypted secret via KMS');
}
// vuln-code-snippet end php_hardcoded_kms_decrypt
