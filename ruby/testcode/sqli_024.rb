require_relative 'shared'

# vuln-code-snippet start ruby_sqli_stored_taint
def list_inventory_items(req)
  db = get_db_connection
  params = {
    category: req.param('category'),
    warehouse: req.header('X-Warehouse-Id'),
    min_stock: req.param('min_stock', default: '0')
  }
  rows = db.execute("SELECT id, name, quantity, price FROM inventory WHERE category = '#{params[:category]}' AND warehouse_id = #{params[:warehouse]} AND quantity >= #{params[:min_stock]}")  # vuln-code-snippet vuln-line ruby_sqli_stored_taint
  items = rows.map { |r| { id: r[0], name: r[1], quantity: r[2], price: r[3] } }
  BenchmarkResponse.json({ items: items, warehouse: params[:warehouse] })
end
# vuln-code-snippet end ruby_sqli_stored_taint
