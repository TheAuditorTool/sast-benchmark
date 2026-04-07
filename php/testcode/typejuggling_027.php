<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_preg_match_loose
function typejuggling027(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('code');
    $result = preg_match('/^[a-z]+$/', $input);
    if ($result == 0) { // vuln-code-snippet vuln-line php_tj_preg_match_loose
        return BenchmarkResponse::ok('no match');
    }
    return BenchmarkResponse::ok('match');
}
// vuln-code-snippet end php_tj_preg_match_loose
