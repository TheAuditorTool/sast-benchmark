<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_phar_wrapper
function deserial_phar_wrapper(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $content = file_get_contents("phar://" . $file); // vuln-code-snippet vuln-line php_deser_phar_wrapper
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_deser_phar_wrapper
