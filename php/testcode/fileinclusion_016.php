<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_open_basedir
function fileinclusion016(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    $templateDir = __DIR__ . '/templates/';
    ini_set('open_basedir', $templateDir);
    include $templateDir . $page . '.php'; // vuln-code-snippet safe-line php_fi_open_basedir
    return BenchmarkResponse::ok('Page rendered');
}
// vuln-code-snippet end php_fi_open_basedir
