<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_include_get
function fileinclusion001(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    include($page . ".php"); // vuln-code-snippet vuln-line php_fi_include_get
    return BenchmarkResponse::ok("page loaded");
}
// vuln-code-snippet end php_fi_include_get
