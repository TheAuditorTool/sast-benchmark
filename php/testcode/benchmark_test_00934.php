<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00934(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $smarty = new Smarty();
    $smarty->display('string:' . $input);
    return BenchmarkResponse::ok('rendered');
}
