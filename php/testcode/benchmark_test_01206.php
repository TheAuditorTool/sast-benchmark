<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01206(BenchmarkRequest $req): BenchmarkResponse {
    $attrs = $_POST;
    $model = new stdClass();
    foreach ($attrs as $key => $val) {
        $model->$key = $val;
    }
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("INSERT INTO products (name, price, stock, is_featured) VALUES (?, ?, ?, ?)");
    $stmt->execute([$model->name, $model->price, $model->stock, $model->is_featured]);
    return BenchmarkResponse::ok('product created');
}
