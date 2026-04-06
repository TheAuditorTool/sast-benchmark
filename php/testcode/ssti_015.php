<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_ob_include
function ssti015(BenchmarkRequest $req): BenchmarkResponse {
    $name = htmlspecialchars($req->param('name'), ENT_QUOTES, 'UTF-8');
    ob_start();
    include __DIR__ . '/templates/greeting.php'; // vuln-code-snippet safe-line php_ssti_ob_include
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_ob_include
