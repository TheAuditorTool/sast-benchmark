<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00649(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $smarty->setTemplateDir('/templates');
    $smarty->assign('title', htmlspecialchars($req->param('title'), ENT_QUOTES, 'UTF-8'));
    $output = $smarty->fetch('page.tpl');
    return BenchmarkResponse::html($output);
}
