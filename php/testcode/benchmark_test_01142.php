<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01142(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $title = $req->post('title');
    $author = $req->post('author');
    $conn->query("INSERT INTO articles (title, author) VALUES ('" . $title . "', '" . $author . "')");
    $conn->close();
    return BenchmarkResponse::ok('created');
}
