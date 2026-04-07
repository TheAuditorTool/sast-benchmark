<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00138(BenchmarkRequest $req): BenchmarkResponse {
    $userInput = $req->param('name');
    ob_start();
    printf("%s", $userInput);
    $html = ob_get_clean();
    return BenchmarkResponse::html($html);
}
