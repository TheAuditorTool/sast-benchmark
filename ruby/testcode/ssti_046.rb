require_relative 'shared'

# vuln-code-snippet start ruby_ssti_actionview_partial_fixed
def handler(req)
  # Simulates ActionView render with a fixed partial path; user input only flows into locals hash.
  # partial name is a string literal — no user-controlled template source.
  locals = { user: { name: ERB::Util.html_escape(req.param('name')) } }
  render_result = "render(partial: 'users/row', locals: #{locals})" # vuln-code-snippet safe-line ruby_ssti_actionview_partial_fixed
  BenchmarkResponse.html(render_result)
end
# vuln-code-snippet end ruby_ssti_actionview_partial_fixed
