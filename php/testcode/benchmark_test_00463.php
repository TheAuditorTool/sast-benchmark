<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00463(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $ip = gethostbyname($host);
    if (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE) === false) {
        return BenchmarkResponse::badRequest('Blocked');
    }
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_RESOLVE, ["$host:443:$ip", "$host:80:$ip"]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
