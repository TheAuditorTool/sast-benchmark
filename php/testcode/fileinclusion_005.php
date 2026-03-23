<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_include_once_tainted
function fileinclusion005(BenchmarkRequest $req): BenchmarkResponse {
    $theme = $req->param('theme');
    include_once("themes/" . $theme . "/header.php"); // vuln-code-snippet vuln-line php_fi_include_once_tainted
    return BenchmarkResponse::ok("theme loaded");
}
// vuln-code-snippet end php_fi_include_once_tainted
