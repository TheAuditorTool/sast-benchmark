<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_plates_native
function ssti014(BenchmarkRequest $req): BenchmarkResponse {
    $templates = new League\Plates\Engine(__DIR__ . '/templates');
    $output = $templates->render('profile', ['name' => $req->param('name')]); // vuln-code-snippet safe-line php_ssti_plates_native
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_plates_native
