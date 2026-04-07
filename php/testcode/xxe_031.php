<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_rss_feed_url
function xxe031(BenchmarkRequest $req): BenchmarkResponse {
    $feedUrl = $req->param('url');
    $data = simplexml_load_file($feedUrl); // vuln-code-snippet vuln-line php_xxe_rss_feed_url
    return BenchmarkResponse::ok((string)$data->channel->title);
}
// vuln-code-snippet end php_xxe_rss_feed_url
