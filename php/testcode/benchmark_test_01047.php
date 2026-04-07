<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01047(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $cart = unserialize($_SESSION['cart']);
    $item = $req->param('item');
    $cart[] = $item;
    $_SESSION['cart'] = serialize($cart);
    return BenchmarkResponse::json(['cart' => $cart]);
}
