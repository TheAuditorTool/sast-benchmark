<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00978(BenchmarkRequest $req): BenchmarkResponse {
    $p = $req->post('prefs') ?? [];
    $prefs = new UserPrefs(lang: $p['lang'] ?? 'en', theme: $p['theme'] ?? 'default');
    return BenchmarkResponse::ok("lang={$prefs->lang}");
}
