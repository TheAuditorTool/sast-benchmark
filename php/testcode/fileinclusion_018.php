<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_zip_wrapper
function fileinclusion018(BenchmarkRequest $req): BenchmarkResponse {
    $archive = $req->param('archive');
    $entry   = $req->param('entry');
    include 'zip://' . $archive . '#' . $entry; // vuln-code-snippet vuln-line php_fi_zip_wrapper
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_zip_wrapper
