require_relative '../../../testcode/shared'
require 'erb'
require 'base64'
require 'openssl'

# Rails API - SearchController
# Covers: ssti, hardcodedcreds

# vuln-code-snippet start ra_ssti_erb_from_string
def search_render_template(req)
  user_template = req.param('template')
  query = req.param('q')
  output = ERB.new(user_template).result(binding) # vuln-code-snippet vuln-line ra_ssti_erb_from_string
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ra_ssti_erb_from_string

# vuln-code-snippet start ra_ssti_file_template
def search_render_safe(req)
  query = req.param('q')
  template_path = File.join(File.dirname(__FILE__), '..', 'views', 'search.html.erb')
  output = ERB.new(File.read(template_path)).result(binding) # vuln-code-snippet safe-line ra_ssti_file_template
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ra_ssti_file_template

# vuln-code-snippet start ra_hardcoded_jwt
def search_generate_token(req)
  user_id = req.param('user_id')
  secret = 'hardcoded-jwt-secret-key-do-not-use' # vuln-code-snippet vuln-line ra_hardcoded_jwt
  header = Base64.strict_encode64('{"alg":"HS256","typ":"JWT"}')
  payload = Base64.strict_encode64("{\"sub\":\"#{user_id}\"}")
  signature = OpenSSL::HMAC.hexdigest('SHA256', secret, "#{header}.#{payload}")
  token = "#{header}.#{payload}.#{signature}"
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ra_hardcoded_jwt

# vuln-code-snippet start ra_hardcoded_env_jwt
def search_generate_token_safe(req)
  user_id = req.param('user_id')
  secret = ENV.fetch('JWT_SECRET') # vuln-code-snippet safe-line ra_hardcoded_env_jwt
  header = Base64.strict_encode64('{"alg":"HS256","typ":"JWT"}')
  payload = Base64.strict_encode64("{\"sub\":\"#{user_id}\"}")
  signature = OpenSSL::HMAC.hexdigest('SHA256', secret, "#{header}.#{payload}")
  token = "#{header}.#{payload}.#{signature}"
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ra_hardcoded_env_jwt
