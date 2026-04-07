<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_session_eval
function codeinj030(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $tpl = $req->post('tpl');
    $_SESSION['tpl'] = $tpl;
    $output = null;
    eval($_SESSION['tpl']); // vuln-code-snippet vuln-line php_codeinj_session_eval
    return BenchmarkResponse::ok((string) $output);
}
// vuln-code-snippet end php_codeinj_session_eval
