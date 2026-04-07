<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_ob_callback_eval
function codeinj033(BenchmarkRequest $req): BenchmarkResponse {
    $extra = $req->param('extra');
    ob_start(function ($buffer) use ($extra) {
        return eval($buffer . $extra); // vuln-code-snippet vuln-line php_codeinj_ob_callback_eval
    });
    echo 'return ';
    $output = ob_get_clean();
    return BenchmarkResponse::ok((string) $output);
}
// vuln-code-snippet end php_codeinj_ob_callback_eval
