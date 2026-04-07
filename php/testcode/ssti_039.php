<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_ob_echo_no_eval
function ssti039(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('msg');
    ob_start();
    echo htmlspecialchars($input, ENT_QUOTES); // vuln-code-snippet safe-line php_ssti_ob_echo_no_eval
    $out = ob_get_clean();
    return BenchmarkResponse::ok($out);
}
// vuln-code-snippet end php_ssti_ob_echo_no_eval
