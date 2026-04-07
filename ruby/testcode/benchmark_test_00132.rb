require_relative 'shared'

def format_for_display(val)
  val.strip.gsub(/<[^>]*>/, '')
end

def fetch_product_raw(req)
  raw_id = req.param('id')
  display_id = format_for_display(raw_id)
  db = get_db_connection
  rows = db.execute("SELECT * FROM products WHERE id = #{display_id}")
  BenchmarkResponse.json({ product: rows.first })
end
