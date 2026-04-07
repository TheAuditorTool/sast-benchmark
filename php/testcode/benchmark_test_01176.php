<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01176(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->param('template');
    $content = file_get_contents('/app/templates/' . $template . '.html');
    return BenchmarkResponse::html($content);
}
