<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00656(BenchmarkRequest $req): BenchmarkResponse {
    $prefs = $req->cookie('prefs');
    parse_str($prefs);
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
