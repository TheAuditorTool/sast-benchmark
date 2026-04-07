require_relative 'shared'

def render_comment_haml(req)
  comment = req.param('comment')
  html = haml_render("!= comment", locals: { comment: comment })
  BenchmarkResponse.html(html)
end
