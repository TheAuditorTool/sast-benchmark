<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01170(BenchmarkRequest $req): BenchmarkResponse {
    $comment = $req->post('comment');
    $author = $req->post('author');
    $c = htmlspecialchars($comment, ENT_QUOTES, 'UTF-8');
    $a = htmlspecialchars($author, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html('<blockquote>' . $c . '</blockquote><cite>' . $a . '</cite>');
}
