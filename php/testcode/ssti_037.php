<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_smarty_compiled_file
function ssti037(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $smarty->display('compiled/user_card.tpl'); // vuln-code-snippet safe-line php_ssti_smarty_compiled_file
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_smarty_compiled_file
