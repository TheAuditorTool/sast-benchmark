<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01171(BenchmarkRequest $req): BenchmarkResponse {
    $title = $req->param('title');
    $content = $req->param('content');
    $t = htmlspecialchars($title, ENT_QUOTES, 'UTF-8');
    $c = htmlspecialchars($content, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html("<article><h2>$t</h2><p>$c</p></article>");
}
