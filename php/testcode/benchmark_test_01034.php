<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01034(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $out);
    $allowed = ['lang', 'theme'];
    $validated = array_intersect_key($out, array_flip($allowed));
    $lang = $validated['lang'] ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
