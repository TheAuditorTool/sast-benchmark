require_relative 'shared'

def handler(req)
  locals = { user: { name: ERB::Util.html_escape(req.param('name')) } }
  render_result = "render(partial: 'users/row', locals: #{locals})"
  BenchmarkResponse.html(render_result)
end
