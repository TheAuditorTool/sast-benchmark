<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00251(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ip = gethostbyname(parse_url($url, PHP_URL_HOST));
    if (strpos($ip, '192.168.') === 0) {
        return BenchmarkResponse::badRequest('internal');
    }
    $content = file_get_contents($req->param('url'));
    return BenchmarkResponse::ok($content);
}
