<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_match_strict
function typejuggling004(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    $result = match ($role) { // vuln-code-snippet safe-line php_tj_match_strict
        'admin' => 'admin access granted',
        'editor' => 'editor access',
        default => 'guest access',
    };
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_tj_match_strict
