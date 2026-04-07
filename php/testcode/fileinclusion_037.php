<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_open_basedir_restriction
function fileinclusion037(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('open_basedir', '/var/app/templates'); // vuln-code-snippet safe-line php_fi_open_basedir_restriction
    $tpl = basename($req->param('tpl'));
    include '/var/app/templates/' . $tpl . '.php';
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_open_basedir_restriction
