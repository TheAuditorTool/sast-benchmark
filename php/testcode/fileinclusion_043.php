<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_basename_dir_lock
function fileinclusion043(BenchmarkRequest $req): BenchmarkResponse {
    $tpl  = basename($req->param('tpl')); // vuln-code-snippet safe-line php_fi_basename_dir_lock
    include TEMPLATES_DIR . $tpl . '.php';
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_basename_dir_lock
