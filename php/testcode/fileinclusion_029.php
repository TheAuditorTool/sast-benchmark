<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_remote_cdn_include
function fileinclusion029(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    include 'http://cdn.example.com/' . $tpl; // vuln-code-snippet vuln-line php_fi_remote_cdn_include
    return BenchmarkResponse::ok('Template loaded');
}
// vuln-code-snippet end php_fi_remote_cdn_include
