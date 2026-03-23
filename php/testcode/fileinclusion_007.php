<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_dynamic_class
function fileinclusion007(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    include("classes/" . $className . ".php"); // vuln-code-snippet vuln-line php_fi_dynamic_class
    return BenchmarkResponse::ok("class loaded");
}
// vuln-code-snippet end php_fi_dynamic_class
