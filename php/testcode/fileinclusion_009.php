<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_null_byte
function fileinclusion009(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    include("uploads/" . $file . ".php"); // vuln-code-snippet vuln-line php_fi_null_byte
    return BenchmarkResponse::ok("file loaded");
}
// vuln-code-snippet end php_fi_null_byte
