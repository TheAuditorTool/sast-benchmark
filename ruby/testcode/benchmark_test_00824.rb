require_relative 'shared'

def handler(req)
  search_term = req.param('q')
  category = req.param('category', default: 'all')
  safe_term = h(search_term)
  safe_cat = h(category)
  body = "<p>Results for <em>#{safe_term}</em> in <strong>#{safe_cat}</strong></p>"
  BenchmarkResponse.html(body)
end
