<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01163(BenchmarkRequest $req): BenchmarkResponse {
    $title = $req->param('title');
    $content = $req->param('content');
    $html = "<article><h2>$title</h2><p>$content</p></article>";
    return BenchmarkResponse::html($html);
}
