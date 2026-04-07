from module_b import get_db, get_stock

def place_order(product_id, quantity, user_id):
    if quantity <= 0:
        return {"error": "Quantity must be positive"}, 400

    if get_stock(product_id) is None:
        return {"error": "Product not found"}, 404

    conn = get_db()

    cursor = conn.execute(
        "UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?",
        (quantity, product_id, quantity),
    )

    if cursor.rowcount == 0:
        conn.close()
        return {"error": "Insufficient stock"}, 422

    conn.execute(
        "INSERT INTO orders (product_id, quantity, user_id) VALUES (?, ?, ?)",
        (product_id, quantity, user_id),
    )
    conn.commit()
    conn.close()
    return {"status": "ok", "ordered": quantity}, 200
