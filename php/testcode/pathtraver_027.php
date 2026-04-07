<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_phardata_extract
function pathtraver027(BenchmarkRequest $req): BenchmarkResponse {
    $pharPath = $req->param('phar');
    $webroot = '/var/www/html/';
    $phar = new PharData($pharPath);
    $phar->extractTo($webroot); // vuln-code-snippet vuln-line php_pt_phardata_extract
    return BenchmarkResponse::ok('extracted');
}
// vuln-code-snippet end php_pt_phardata_extract
