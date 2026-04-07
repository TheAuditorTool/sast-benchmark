<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xmlreader_user_path
function xxe020(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $reader = new XMLReader();
    $reader->open($path); // vuln-code-snippet vuln-line php_xxe_xmlreader_user_path
    return BenchmarkResponse::ok('opened');
}
// vuln-code-snippet end php_xxe_xmlreader_user_path
