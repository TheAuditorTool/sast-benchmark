<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_ssm_param
function hardcodedcreds012(BenchmarkRequest $req): BenchmarkResponse {
    $client = new Aws\Ssm\SsmClient(['region' => 'us-east-1', 'version' => 'latest']);
    $result = $client->getParameter([
        'Name' => '/myapp/db/password',
        'WithDecryption' => true,
    ]);
    $password = $result['Parameter']['Value']; // vuln-code-snippet safe-line php_hardcoded_ssm_param
    return BenchmarkResponse::ok('Retrieved secret from SSM');
}
// vuln-code-snippet end php_hardcoded_ssm_param
