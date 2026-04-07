<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00527(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $smarty->display('compiled/user_card.tpl');
    return BenchmarkResponse::ok('rendered');
}
