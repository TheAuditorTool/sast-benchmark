require_relative 'shared'

# vuln-code-snippet start ruby_xss_h_helper
def xss_h_helper(req)
  search_term = req.param('q')
  category = req.param('category', default: 'all')
  safe_term = h(search_term) # vuln-code-snippet safe-line ruby_xss_h_helper
  safe_cat = h(category)
  body = "<p>Results for <em>#{safe_term}</em> in <strong>#{safe_cat}</strong></p>"
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end ruby_xss_h_helper
