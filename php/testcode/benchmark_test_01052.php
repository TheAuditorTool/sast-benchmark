<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01052(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('greeting');

    $html = <<<HTML
    <html><body><p>{$input}</p></body></html>
    HTML;

    return BenchmarkResponse::html($html);
}
