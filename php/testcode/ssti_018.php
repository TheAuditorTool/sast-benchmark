<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_smarty_string_display
function ssti018(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $smarty = new Smarty();
    $smarty->display('string:' . $input); // vuln-code-snippet vuln-line php_ssti_smarty_string_display
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_smarty_string_display
