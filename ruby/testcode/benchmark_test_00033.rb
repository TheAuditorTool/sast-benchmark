require_relative 'shared'

def render_rich_text(req)
  body = req.param('body')
  clean = sanitize(body, tags: %w[p b i em strong], attributes: %w[class])
  BenchmarkResponse.html("<article>#{clean}</article>")
end
