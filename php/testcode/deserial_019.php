<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_session_cart
function deserial019(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $cart = unserialize($_SESSION['cart']); // vuln-code-snippet vuln-line php_deser_session_cart
    $item = $req->param('item');
    $cart[] = $item;
    $_SESSION['cart'] = serialize($cart);
    return BenchmarkResponse::json(['cart' => $cart]);
}
// vuln-code-snippet end php_deser_session_cart
