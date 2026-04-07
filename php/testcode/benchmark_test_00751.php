<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00751(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    $result = (function () use ($input) {
        parse_str($input, $local);
        return $local['key'] ?? null;
    })();
    return BenchmarkResponse::ok((string) $result);
}
