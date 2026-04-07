<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00885(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = '{php}' . $req->param('code') . '{/php}';
    $smarty = new Smarty();
    $smarty->display('string:' . $tpl);
    return BenchmarkResponse::ok('rendered');
}
