<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00651(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('color');
    $html = "<div style=\"color: $input\">Hello</div>";
    return BenchmarkResponse::html($html);
}
