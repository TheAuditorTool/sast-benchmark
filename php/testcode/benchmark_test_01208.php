<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01208(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $price = $req->post('price');
    $stock = $req->post('stock');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("INSERT INTO products (name, price, stock) VALUES (?, ?, ?)");
    $stmt->execute([$name, $price, $stock]);
    return BenchmarkResponse::ok('product created');
}
