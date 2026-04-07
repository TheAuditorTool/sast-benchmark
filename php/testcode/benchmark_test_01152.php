<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01152(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $title = $req->post('title');
    $author = $req->post('author');
    $stmt = $conn->prepare("INSERT INTO articles (title, author) VALUES (?, ?)");
    $stmt->bind_param("ss", $title, $author);
    $stmt->execute();
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::ok('created');
}
