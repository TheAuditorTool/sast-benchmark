<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00749(BenchmarkRequest $req): BenchmarkResponse {
    $smarty = new Smarty();
    $tpl = $req->post('tpl');
    $output = $smarty->fetch("string:" . $tpl);
    return BenchmarkResponse::html($output);
}
