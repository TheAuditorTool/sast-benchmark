<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00925(BenchmarkRequest $req): BenchmarkResponse {
    $templateMap = [md5('home') => 'home.php', md5('about') => 'about.php'];
    $key  = $req->param('key');
    $file = $templateMap[md5($key)] ?? 'default.php';
    include __DIR__ . '/pages/' . $file;
    return BenchmarkResponse::ok('Rendered');
}
