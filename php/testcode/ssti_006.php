<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_smarty_file
function ssti006(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $smarty->setTemplateDir('/templates');
    $smarty->assign('title', htmlspecialchars($req->param('title'), ENT_QUOTES, 'UTF-8'));
    $output = $smarty->fetch('page.tpl'); // vuln-code-snippet safe-line php_ssti_smarty_file
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_smarty_file
