<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00376(BenchmarkRequest $req): BenchmarkResponse {
    $map = ['list' => 'list.php', 'detail' => 'detail.php'];
    $key = $req->param('key');
    $tpl = $map[$key] ?? 'default.php';
    include __DIR__ . '/pages/' . $tpl;
    return BenchmarkResponse::ok('Rendered');
}
