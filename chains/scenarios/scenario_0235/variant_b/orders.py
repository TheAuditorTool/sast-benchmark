from inventory import get_db, get_stock

def place_order(product_id, quantity, user_id):
    if quantity <= 0:
        return {"error": "Quantity must be positive"}, 400

    stock = get_stock(product_id)
    if stock is None:
        return {"error": "Product not found"}, 404

    if stock < quantity:
        return {"error": "Insufficient stock"}, 422

    conn = get_db()

# vuln-code-snippet start ChainScenario0235B
    conn.execute(
        "UPDATE products SET stock = stock - ? WHERE id = ?",
        (quantity, product_id),
    )  # vuln-code-snippet target-line ChainScenario0235B
# vuln-code-snippet end ChainScenario0235B

    conn.execute(
        "INSERT INTO orders (product_id, quantity, user_id) VALUES (?, ?, ?)",
        (product_id, quantity, user_id),
    )
    conn.commit()
    conn.close()
    return {"status": "ok", "ordered": quantity}, 200
