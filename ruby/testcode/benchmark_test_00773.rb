require_relative 'shared'

def render_bio_slim(req)
  bio = req.param('bio')
  html = slim_render("== bio", locals: { bio: bio })
  BenchmarkResponse.html(html)
end
