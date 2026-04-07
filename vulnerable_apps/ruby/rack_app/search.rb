require_relative '../../testcode/shared'
require 'erb'

# rack_app - Search and template rendering

# vuln-code-snippet start rk_ssti_erb
def search_results(env)
  req = Rack::Request.new(env)
  query = req.params['q']
  output = ERB.new(query).result(binding) # vuln-code-snippet vuln-line rk_ssti_erb
  [200, { 'Content-Type' => 'text/html; charset=UTF-8' }, [output]]
end
# vuln-code-snippet end rk_ssti_erb

# vuln-code-snippet start rk_ssti_file
def search_results_safe(env)
  req = Rack::Request.new(env)
  query = escape_html(req.params['q'])
  template_src = File.read('search_results.erb') # vuln-code-snippet safe-line rk_ssti_file
  output = ERB.new(template_src).result(binding)
  [200, { 'Content-Type' => 'text/html; charset=UTF-8' }, [output]]
end
# vuln-code-snippet end rk_ssti_file
