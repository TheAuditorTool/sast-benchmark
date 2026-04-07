from coupons import get_db, get_coupon

def redeem_coupon(code, user_id):
    coupon = get_coupon(code)
    if not coupon:
        return {"error": "Coupon not found"}, 404

    if coupon["used"] != 0:
        return {"error": "Coupon already redeemed"}, 409

    conn = get_db()

# vuln-code-snippet start ChainScenario0133B
    conn.execute(
        "UPDATE coupons SET used = 1 WHERE code = ?", (code,)
    )  # vuln-code-snippet target-line ChainScenario0133B
# vuln-code-snippet end ChainScenario0133B

    conn.execute(
        "INSERT INTO redemptions (coupon_code, user_id) VALUES (?, ?)",
        (code, user_id),
    )
    conn.commit()
    conn.close()

    return {"status": "ok", "discount_pct": coupon["discount_pct"]}, 200
