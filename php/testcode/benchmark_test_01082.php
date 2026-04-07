<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01082(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $tpl = 'Hello {name}';
    $result = preg_replace_callback('/' . $pattern . '/', fn($m) => eval('return "' . $m[0] . '";'), $tpl);
    return BenchmarkResponse::ok($result);
}
