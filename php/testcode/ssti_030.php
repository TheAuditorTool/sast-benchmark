<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_smarty_php_block
function ssti030(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = '{php}' . $req->param('code') . '{/php}';
    $smarty = new Smarty();
    $smarty->display('string:' . $tpl); // vuln-code-snippet vuln-line php_ssti_smarty_php_block
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_smarty_php_block
