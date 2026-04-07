<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_plates_static_dir
function ssti036(BenchmarkRequest $req): BenchmarkResponse {
    $templates = new League\Plates\Engine(__DIR__ . '/views');
    $html = $templates->render('profile', ['user' => $req->param('name')]); // vuln-code-snippet safe-line php_ssti_plates_static_dir
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_plates_static_dir
