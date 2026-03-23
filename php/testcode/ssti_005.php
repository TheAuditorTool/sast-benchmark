<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_smarty_string
function ssti005(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $tpl = $req->post('tpl');
    $output = $smarty->fetch("string:" . $tpl); // vuln-code-snippet vuln-line php_ssti_smarty_string
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_smarty_string
