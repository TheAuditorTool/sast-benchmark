<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01036(BenchmarkRequest $req): BenchmarkResponse {
    $client = new Aws\Ssm\SsmClient(['region' => 'us-east-1', 'version' => 'latest']);
    $result = $client->getParameter([
        'Name' => '/myapp/db/password',
        'WithDecryption' => true,
    ]);
    $password = $result['Parameter']['Value'];
    return BenchmarkResponse::ok('Retrieved secret from SSM');
}
