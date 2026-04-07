<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_plates_user_path
function ssti028(BenchmarkRequest $req): BenchmarkResponse {
    $userTpl = $req->param('template');
    $templates = new League\Plates\Engine('/views');
    echo $templates->render($userTpl, []); // vuln-code-snippet vuln-line php_ssti_plates_user_path
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_plates_user_path
