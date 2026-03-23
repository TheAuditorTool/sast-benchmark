<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_blade_safe
function ssti004(BenchmarkRequest $req): BenchmarkResponse {
    $user = $req->param('user');
    $viewFile = __DIR__ . '/views/profile.blade.php';
    $data = ['user' => htmlspecialchars($user, ENT_QUOTES, 'UTF-8')]; // vuln-code-snippet safe-line php_ssti_blade_safe
    ob_start();
    extract($data);
    include $viewFile;
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_blade_safe
