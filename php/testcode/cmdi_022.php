<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_imagick_shell
function cmdi022(BenchmarkRequest $req): BenchmarkResponse {
    $userInput = $req->param('file');
    $img = new Imagick($userInput . '.jpg'); // vuln-code-snippet vuln-line php_cmdi_imagick_shell
    $img->scaleImage(100, 100);
    return BenchmarkResponse::ok('resized');
}
// vuln-code-snippet end php_cmdi_imagick_shell
