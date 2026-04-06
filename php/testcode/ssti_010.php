<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_eval_template
function ssti010(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    ob_start();
    eval("?>" . $template); // vuln-code-snippet vuln-line php_ssti_eval_template
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_eval_template
