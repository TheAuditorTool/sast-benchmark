require_relative 'shared'

def render_plain_comment(req)
  comment = req.param('comment')
  plain_text = strip_tags(comment)
  BenchmarkResponse.html("<p>#{h(plain_text)}</p>")
end
