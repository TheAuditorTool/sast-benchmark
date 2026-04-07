<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_uuid4_random_int
function weakrand041(BenchmarkRequest $req): BenchmarkResponse {
    $uuid = sprintf( // vuln-code-snippet safe-line php_weakrand_uuid4_random_int
        '%04x%04x-%04x-%04x-%04x-%04x%04x%04x',
        random_int(0, 0xffff), random_int(0, 0xffff),
        random_int(0, 0xffff),
        random_int(0x4000, 0x4fff),
        random_int(0x8000, 0xbfff),
        random_int(0, 0xffff), random_int(0, 0xffff), random_int(0, 0xffff)
    );
    return BenchmarkResponse::ok($uuid);
}
// vuln-code-snippet end php_weakrand_uuid4_random_int
