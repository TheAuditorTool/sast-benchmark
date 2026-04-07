<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_expect_wrapper
function fileinclusion021(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    include 'expect://' . $cmd; // vuln-code-snippet vuln-line php_fi_expect_wrapper
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_expect_wrapper
