<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_aws_keypair
function hardcodedcreds023(BenchmarkRequest $req): BenchmarkResponse {
    define('AWS_ACCESS_KEY', 'AKIAIOSFODNN7EXAMPLE');
    define('AWS_SECRET', 'wJalrXUtnFEMI/K7MDENG'); // vuln-code-snippet vuln-line php_hardcoded_aws_keypair
    $bucket = $req->param('bucket');
    $headers = [
        'X-Amz-Access-Key: ' . AWS_ACCESS_KEY,
        'X-Amz-Secret: ' . AWS_SECRET,
    ];
    return BenchmarkResponse::ok('bucket: ' . $bucket);
}
// vuln-code-snippet end php_hardcoded_aws_keypair
