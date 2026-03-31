require_relative 'shared'

# vuln-code-snippet start ruby_xss_render_inline
def xss_render_inline(req)
  template_fragment = req.param('tpl')
  user_name = req.param('user')
  erb_src = "<div>Hello <%= #{template_fragment} %>, #{user_name}</div>" # vuln-code-snippet vuln-line ruby_xss_render_inline
  require 'erb'
  renderer = ERB.new(erb_src)
  output = renderer.result(binding)
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_xss_render_inline
