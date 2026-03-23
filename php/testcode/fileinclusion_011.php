<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_remote_url
function fileinclusion011(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    include($url); // vuln-code-snippet vuln-line php_fi_remote_url
    return BenchmarkResponse::ok("remote content loaded");
}
// vuln-code-snippet end php_fi_remote_url
