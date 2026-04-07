<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_match_strict_php8
function typejuggling038(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('type');
    $result = match($input) { // vuln-code-snippet safe-line php_tj_match_strict_php8
        '1' => 'admin',
        '2' => 'user',
        default => 'guest',
    };
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_tj_match_strict_php8
