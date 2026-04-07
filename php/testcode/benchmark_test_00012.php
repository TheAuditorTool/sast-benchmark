<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00012(BenchmarkRequest $req): BenchmarkResponse {
    $input = ['lang' => $req->param('lang'), 'theme' => $req->param('theme')];
    extract($input, EXTR_PREFIX_ALL, 'usr_');
    $lang = $usr_lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
