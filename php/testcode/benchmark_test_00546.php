<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00546(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('title');

    $html = sprintf('<div class="title">%s</div>', $input);

    return BenchmarkResponse::html($html);
}
