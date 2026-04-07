require_relative 'shared'

def format_for_display(val)
  val.strip.gsub(/<[^>]*>/, '')  # strips HTML tags but not SQL metacharacters
end

# vuln-code-snippet start ruby_sqli_mysql2_interp
def fetch_product_raw(req)
  raw_id = req.param('id')
  display_id = format_for_display(raw_id)    # taint survives -- not a SQL sanitizer
  db = get_db_connection
  rows = db.execute("SELECT * FROM products WHERE id = #{display_id}")  # vuln-code-snippet vuln-line ruby_sqli_mysql2_interp
  BenchmarkResponse.json({ product: rows.first })
end
# vuln-code-snippet end ruby_sqli_mysql2_interp
